//! The Lua runner: `tests.lua` in an embedded Lua 5.2 (Factorio's version).
//!
//! Layout of a workspace: `tests.lua` (the spec), `src/*.lua` (what the learner edits),
//! optionally `mod/` for Factorio quests. Modules resolve from `src/`, the workspace root,
//! then the track's `lib/`. Output is shaped like cargo's so `grader::parse` serves both.

use std::fs;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use mlua::{Function, HookTriggers, Lua, Table, Value, VmState};
use regex::Regex;

use crate::content::Objective;
use crate::grader::{CheckResult, parse};

pub const HARNESS: &str = include_str!("../lua/test.lua");
/// Wall-clock budget for one tests.lua run (kills `while true do end`).
const TIME_LIMIT: Duration = Duration::from_secs(5);

/// Grade `workspace` and map results to `objectives`.
pub fn run(workspace: &Path, lib_dir: Option<&Path>, objectives: &[Objective]) -> Result<CheckResult> {
    let start = Instant::now();
    let ws = workspace.to_path_buf();
    let lib = lib_dir.map(Path::to_path_buf);
    // Own thread: the Lua state is not Send, and a runaway script must not block callers.
    // mlua's error is not Send either, so it is rendered to text before leaving the thread.
    let (stdout, stderr, exit_ok) = std::thread::spawn(move || match run_inner(&ws, lib.as_deref()) {
        Ok(x) => x,
        Err(e) => (String::new(), format!("error: {}
", clean_error(&e)), false),
    })
    .join()
    .map_err(|_| anyhow!("the Lua runner thread panicked"))?;
    let mut r = parse(&stdout, &stderr, exit_ok, objectives);
    // Lua errors carry `file:line:`; lift that into the diagnostic.
    let loc = Regex::new(r"(?m)^(?:error: )?([\w./\\-]+\.lua):(\d+):").unwrap();
    for d in r.diagnostics.iter_mut().filter(|d| d.file.is_none()) {
        if let Some(c) = loc.captures(&d.rendered) {
            d.file = Some(c[1].to_string());
            d.line = c[2].parse().ok();
            d.message = d.rendered.lines().next().unwrap_or("").trim_start_matches("error: ").to_string();
        }
    }
    r.duration_ms = start.elapsed().as_millis() as u64;
    Ok(r)
}

/// Run one script (e.g. `src/main.lua`) with `print` captured; `(output, error)`.
pub fn run_script(ws: &Path, lib: Option<&Path>, rel: &str) -> (String, Option<String>) {
    let ws = ws.to_path_buf();
    let lib = lib.map(Path::to_path_buf);
    let rel = rel.to_string();
    std::thread::spawn(move || -> (String, Option<String>) {
        let lua = Lua::new();
        let out = std::rc::Rc::new(std::cell::RefCell::new(String::new()));
        let setup = (|| -> mlua::Result<()> {
            sandbox(&lua)?;
            install_searcher(&lua, &ws, lib.as_deref())?;
            let out2 = out.clone();
            let print = lua.create_function(move |_, args: mlua::Variadic<Value>| {
                let mut buf = out2.borrow_mut();
                let parts: Vec<String> = args.iter().map(|v| match v {
                    Value::String(s) => s.to_string_lossy(),
                    other => format!("{other:#?}").trim_matches('"').to_string(),
                }).collect();
                buf.push_str(&parts.join("\t"));
                buf.push('\n');
                Ok(())
            })?;
            lua.globals().set("print", print)?;
            let deadline = Instant::now() + TIME_LIMIT;
            let _ = lua.set_hook(HookTriggers::new().every_nth_instruction(10_000), move |_, _| {
                if Instant::now() > deadline {
                    Err(mlua::Error::runtime("time limit of 5s hit (infinite loop?)"))
                } else {
                    Ok(VmState::Continue)
                }
            });
            Ok(())
        })();
        if let Err(e) = setup {
            return (String::new(), Some(clean_error(&e)));
        }
        let path = ws.join(&rel);
        let src = match fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => return (String::new(), Some(format!("{rel}: {e}"))),
        };
        let err = lua.load(&src).set_name(format!("@{rel}")).exec().err().map(|e| clean_error(&e));
        let text = out.borrow().clone();
        (text, err)
    })
    .join()
    .unwrap_or_else(|_| (String::new(), Some("the Lua thread panicked".into())))
}

/// `(stdout, stderr, ok)`: `ok` is false when tests.lua itself failed to run.
/// mlua's error type is not `Send`, so this stays in mlua's `Result` until `run` converts it.
fn run_inner(ws: &Path, lib: Option<&Path>) -> mlua::Result<(String, String, bool)> {
    let lua = Lua::new();
    sandbox(&lua)?;
    install_searcher(&lua, ws, lib)?;

    // Past the deadline every hook call errors, except for a short grace after each
    // firing so the harness's error handler can run (else "error in error handling").
    let deadline = Instant::now() + TIME_LIMIT;
    let last_fire = std::sync::Mutex::new(None::<Instant>);
    let _ = lua.set_hook(HookTriggers::new().every_nth_instruction(10_000), move |_, _| {
        let now = Instant::now();
        if now <= deadline {
            return Ok(VmState::Continue);
        }
        let mut last = last_fire.lock().unwrap();
        if last.is_some_and(|t| now.duration_since(t) < Duration::from_millis(200)) {
            return Ok(VmState::Continue);
        }
        *last = Some(now);
        Err(mlua::Error::runtime(format!("time limit of {}s hit (infinite loop?)", TIME_LIMIT.as_secs())))
    });

    let tests_path = ws.join("tests.lua");
    let src = fs::read_to_string(&tests_path)
        .map_err(|e| mlua::Error::runtime(format!("reading {}: {e}", tests_path.display())))?;
    if let Err(e) = lua.load(&src).set_name("@tests.lua").exec() {
        return Ok((String::new(), format!("error: {}\n", clean_error(&e)), false));
    }
    let results: Table = lua
        .load("local t = require('oxide.test'); if not t.__ran then t.run() end; return t.__results")
        .eval()?;

    let mut rows: Vec<(String, String, String, String)> = Vec::new();
    for row in results.sequence_values::<Table>() {
        let row = row?;
        rows.push((
            row.get::<String>("name")?,
            row.get::<String>("status")?,
            row.get::<Option<String>>("err")?.unwrap_or_default(),
            row.get::<Option<String>>("out")?.unwrap_or_default(),
        ));
    }
    Ok((format_cargo_style(&rows), String::new(), true))
}

/// Cargo's test output shape, for `grader::parse`.
fn format_cargo_style(rows: &[(String, String, String, String)]) -> String {
    let mut s = format!("running {} tests\n", rows.len());
    for (name, status, ..) in rows {
        s.push_str(&format!("test {name} ... {status}\n"));
    }
    let failed: Vec<_> = rows.iter().filter(|r| r.1 == "FAILED").collect();
    if !failed.is_empty() {
        s.push_str("\nfailures:\n\n");
        for (name, _, err, out) in &failed {
            s.push_str(&format!("---- {name} stdout ----\n"));
            if !out.is_empty() {
                s.push_str(out);
                s.push('\n');
            }
            s.push_str(err);
            s.push_str("\n\n");
        }
        s.push_str("failures:\n");
        for (name, ..) in &failed {
            s.push_str(&format!("    {name}\n"));
        }
    }
    let passed = rows.iter().filter(|r| r.1 == "ok").count();
    let ignored = rows.iter().filter(|r| r.1 == "ignored").count();
    s.push_str(&format!(
        "\ntest result: {}. {passed} passed; {} failed; {ignored} ignored\n",
        if failed.is_empty() { "ok" } else { "FAILED" },
        failed.len()
    ));
    s
}

/// mlua wraps messages in its own text; keep what Lua said.
fn clean_error(e: &mlua::Error) -> String {
    match e {
        mlua::Error::SyntaxError { message, .. } => message.clone(),
        mlua::Error::RuntimeError(m) => m.clone(),
        mlua::Error::CallbackError { cause, .. } => clean_error(cause),
        other => other.to_string(),
    }
}

/// Plain-Lua quests keep `os.time`/`os.clock`; nothing may touch the machine.
fn sandbox(lua: &Lua) -> mlua::Result<()> {
    let g = lua.globals();
    if let Ok(os) = g.get::<Table>("os") {
        for k in ["exit", "execute", "remove", "rename", "tmpname", "getenv", "setlocale"] {
            os.set(k, Value::Nil)?;
        }
    }
    if let Ok(io) = g.get::<Table>("io") {
        for k in ["popen", "open", "lines", "input", "output"] {
            io.set(k, Value::Nil)?;
        }
    }
    g.set("loadfile", Value::Nil)?;
    g.set("dofile", Value::Nil)?;
    Ok(())
}

/// `require` resolves `oxide.test` to the harness, then `src/<name>.lua`,
/// `<name>.lua`, `lib/<name>.lua`, with short chunk names so errors read `src/x.lua:7:`.
fn install_searcher(lua: &Lua, ws: &Path, lib: Option<&Path>) -> mlua::Result<()> {
    let package: Table = lua.globals().get("package")?;
    package.set("path", "")?;
    package.set("cpath", "")?;
    let roots: Vec<(PathBuf, String)> = [(ws.join("src"), "src/".to_string()), (ws.to_path_buf(), String::new())]
        .into_iter()
        .chain(lib.map(|l| (l.to_path_buf(), "lib/".to_string())))
        .collect();
    let searcher = lua.create_function(move |lua, name: String| {
        if name == "oxide.test" {
            let f: Function = lua.load(HARNESS).set_name("@oxide/test.lua").into_function()?;
            return Ok(Value::Function(f));
        }
        let rel = name.replace('.', "/");
        for (root, prefix) in &roots {
            for candidate in [format!("{rel}.lua"), format!("{rel}/init.lua")] {
                let p = root.join(&candidate);
                if p.is_file() {
                    let code = fs::read_to_string(&p).map_err(mlua::Error::external)?;
                    // `@` makes Lua print the name bare: `src/x.lua:3:` not `[string "src/x.lua"]:3:`.
                    let f: Function = lua.load(&code).set_name(format!("@{prefix}{candidate}")).into_function()?;
                    return Ok(Value::Function(f));
                }
            }
        }
        Ok(Value::String(lua.create_string(format!("\n\tno file 'src/{rel}.lua'"))?))
    })?;
    let searchers: Table = package.get("searchers")?;
    // Keep the preload searcher at 1, ours at 2, drop the file searchers.
    searchers.set(2, searcher)?;
    while searchers.raw_len() > 2 {
        searchers.raw_remove(searchers.raw_len())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grader::ObjectiveStatus;

    fn objs(names: &[&str]) -> Vec<Objective> {
        names.iter().map(|n| Objective { test: n.to_string(), label: n.to_string() }).collect()
    }

    fn workspace(src: &str, tests: &str) -> tempfile::TempDir {
        let tmp = tempfile::tempdir().unwrap();
        fs::create_dir_all(tmp.path().join("src")).unwrap();
        fs::write(tmp.path().join("src/arrays.lua"), src).unwrap();
        fs::write(tmp.path().join("tests.lua"), tests).unwrap();
        tmp
    }

    const TESTS: &str = r#"
local t = require("oxide.test")
local arrays = require("arrays")
t.test("sum_one_based", function() t.eq(arrays.sum({1, 2, 3}), 6) end)
t.test("append_and_pop", function()
  local a = {}
  arrays.append(a, 5)
  t.deep_eq(a, {5}, "after append")
end)
t.test("skipped_one", function() t.skip("later") end)
"#;

    #[test]
    fn red_then_green() {
        let red = workspace(
            "local M = {}\nfunction M.sum(xs) return 0 end\nfunction M.append(a, v) end\nreturn M\n",
            TESTS,
        );
        let r = run(red.path(), None, &objs(&["sum_one_based", "append_and_pop"])).unwrap();
        assert!(r.compiled, "{:?}", r.diagnostics);
        assert!(!r.ok);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Failed);
        let out = r.objectives[0].output.as_deref().unwrap();
        assert!(out.contains("expected 6, got 0"), "{out}");
        assert!(out.contains("tests.lua:4:"), "{out}");
        assert!(r.objectives[1].output.as_deref().unwrap().contains("after append"));

        let green = workspace(
            "local M = {}\nfunction M.sum(xs) local s = 0 for _, x in ipairs(xs) do s = s + x end return s end\nfunction M.append(a, v) a[#a + 1] = v end\nreturn M\n",
            TESTS,
        );
        let r = run(green.path(), None, &objs(&["sum_one_based", "append_and_pop"])).unwrap();
        assert!(r.ok, "{}", r.raw_tail);
    }

    #[test]
    fn syntax_error_in_learner_file_is_a_diagnostic() {
        let ws = workspace("local M = {}\nfunction M.sum(xs) return 0\nreturn M\n", TESTS);
        let r = run(ws.path(), None, &objs(&["sum_one_based"])).unwrap();
        assert!(!r.compiled);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::NotRun);
        let d = &r.diagnostics[0];
        assert_eq!(d.file.as_deref(), Some("src/arrays.lua"), "{d:?}");
        assert!(d.line.is_some());
    }

    #[test]
    fn runtime_error_names_the_learner_line() {
        let ws = workspace("local M = {}\nfunction M.sum(xs)\n  return xs.nope.deeper\nend\nfunction M.append() end\nreturn M\n", TESTS);
        let r = run(ws.path(), None, &objs(&["sum_one_based"])).unwrap();
        assert!(r.compiled);
        let out = r.objectives[0].output.as_deref().unwrap();
        assert!(out.contains("src/arrays.lua:3:"), "{out}");
    }

    #[test]
    fn infinite_loop_is_killed() {
        let ws = workspace("local M = {}\nfunction M.sum() while true do end end\nfunction M.append() end\nreturn M\n", TESTS);
        let r = run(ws.path(), None, &objs(&["sum_one_based"])).unwrap();
        assert!(r.compiled);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Failed);
        assert!(r.objectives[0].output.as_deref().unwrap().contains("time limit"));
    }

    #[test]
    fn lib_dir_modules_resolve() {
        let ws = workspace("return {}", "local t = require('oxide.test')\nlocal m = require('mock')\nt.test('uses_lib', function() t.eq(m.answer, 42) end)\n");
        let lib = tempfile::tempdir().unwrap();
        fs::write(lib.path().join("mock.lua"), "return { answer = 42 }").unwrap();
        let r = run(ws.path(), Some(lib.path()), &objs(&["uses_lib"])).unwrap();
        assert!(r.ok, "{}", r.raw_tail);
    }
}
