//! Run `cargo test` in a workspace and turn the output into objectives + diagnostics.
//!
//! Stable cargo has no JSON test output, so: compiler messages come as JSON lines
//! (`--message-format=json-render-diagnostics`), test results as the plain
//! `test name ... ok` lines, both on stdout.

use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::content::{Objective, Quest, Runner};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ObjectiveStatus {
    Passed,
    Failed,
    /// Test never ran: compile error, or the test function is missing.
    NotRun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectiveResult {
    pub test: String,
    pub label: String,
    pub status: ObjectiveStatus,
    /// Captured stdout/panic of a failed test.
    pub output: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    /// `error` | `warning`
    pub level: String,
    pub message: String,
    /// What rustc prints in a terminal, ANSI-free.
    pub rendered: String,
    pub file: Option<String>,
    pub line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    /// Every objective passed and nothing failed to compile.
    pub ok: bool,
    pub compiled: bool,
    pub objectives: Vec<ObjectiveResult>,
    pub diagnostics: Vec<Diagnostic>,
    /// Tests that ran but are not objectives (extra tests in the starter).
    pub other_failures: Vec<String>,
    /// Last part of raw output, for a "show details" drawer.
    pub raw_tail: String,
    pub duration_ms: u64,
}

/// Shared target dir so every quest reuses compiled deps (fast green = good dopamine).
pub fn shared_target_dir(quests_root: &Path) -> PathBuf {
    quests_root.join(".target")
}

fn cargo(workspace: &Path, target_dir: &Path) -> Command {
    let mut c = Command::new("cargo");
    c.current_dir(workspace)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TARGET_DIR", target_dir)
        .env_remove("RUSTFLAGS");
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    c
}

/// Grade a quest's workspace with its runner.
pub fn run_quest(quest: &Quest, workspace: &Path, target_dir: &Path) -> Result<CheckResult> {
    match quest.runner {
        Runner::Cargo => run(workspace, target_dir, &quest.objectives),
        Runner::Lua => crate::lua::run(workspace, quest.lib_dir.as_deref(), &quest.objectives),
        Runner::Gradle => {
            crate::gradle::run(workspace, quest.jdk, &crate::gradle::user_home_for(target_dir), &quest.objectives)
        }
    }
}

/// Whether the "clean" bonus applies: cargo runs clippy; other runners have no linter yet.
pub fn clean_for(quest: &Quest, workspace: &Path, target_dir: &Path) -> bool {
    match quest.runner {
        Runner::Cargo => clippy_clean(workspace, target_dir).unwrap_or(false),
        _ => false,
    }
}

/// How long the tests may run once they are built. A test still running after this is reported and stopped.
pub const TEST_TIME_LIMIT: Duration = Duration::from_secs(60);
/// How long a build may take. Generous: a cold build of a quest with big dependencies takes minutes.
pub const BUILD_TIME_LIMIT: Duration = Duration::from_secs(15 * 60);

/// Run the tests in `workspace` and grade `objectives`.
pub fn run(workspace: &Path, target_dir: &Path, objectives: &[Objective]) -> Result<CheckResult> {
    run_with_limits(workspace, target_dir, objectives, BUILD_TIME_LIMIT, TEST_TIME_LIMIT)
}

/// `run`, with the time limits given. A run that goes over is stopped, and the test that was still running
/// fails with a note saying so, instead of the check spinning forever.
pub fn run_with_limits(workspace: &Path, target_dir: &Path, objectives: &[Objective], build_limit: Duration, test_limit: Duration) -> Result<CheckResult> {
    let start = Instant::now();
    let mut child = cargo(workspace, target_dir)
        .args(["test", "--no-fail-fast", "--message-format=json-render-diagnostics", "--", "--test-threads=1"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("running cargo test (is cargo on PATH?)")?;
    let lines = read_lines(child.stdout.take());
    let errors = read_lines(child.stderr.take());

    let mut stdout = String::new();
    let mut tests_started: Option<Instant> = None;
    let mut over: Option<(&str, Duration)> = None;
    loop {
        match lines.recv_timeout(Duration::from_millis(200)) {
            Ok(line) => {
                // Cargo says when the build is done; only then does the test clock start.
                if tests_started.is_none() && line.starts_with('{') && line.contains("\"reason\":\"build-finished\"") {
                    tests_started = Some(Instant::now());
                }
                stdout.push_str(&line);
                stdout.push('\n');
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
            Err(mpsc::RecvTimeoutError::Timeout) => {}
        }
        over = match tests_started {
            Some(at) if at.elapsed() > test_limit => Some(("tests", test_limit)),
            None if start.elapsed() > build_limit => Some(("build", build_limit)),
            _ => None,
        };
        if over.is_some() {
            kill_tree(&mut child);
            // Whatever is still buffered; a grandchild holding the pipe open must not hang us here.
            while let Ok(line) = lines.recv_timeout(Duration::from_secs(2)) {
                stdout.push_str(&line);
                stdout.push('\n');
            }
            break;
        }
    }
    let status = child.wait().context("waiting for cargo test")?;
    let mut stderr = String::new();
    while let Ok(line) = errors.recv_timeout(Duration::from_millis(500)) {
        stderr.push_str(&line);
        stderr.push('\n');
    }

    let mut r = parse(&stdout, &stderr, status.success(), objectives);
    if let Some((phase, limit)) = over {
        mark_over_time(&mut r, &stdout, phase, limit);
    }
    r.duration_ms = start.elapsed().as_millis() as u64;
    Ok(r)
}

/// Read a pipe line by line on its own thread, so the caller can wait with a time limit.
fn read_lines(pipe: Option<impl std::io::Read + Send + 'static>) -> mpsc::Receiver<String> {
    let (tx, rx) = mpsc::channel();
    if let Some(pipe) = pipe {
        std::thread::spawn(move || {
            for line in BufReader::new(pipe).lines().map_while(Result::ok) {
                if tx.send(line).is_err() {
                    break;
                }
            }
        });
    }
    rx
}

/// Stop cargo and the test program it started. Killing cargo alone leaves the test program running on Windows.
fn kill_tree(child: &mut Child) {
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        let _ = Command::new("taskkill")
            .args(["/F", "/T", "/PID", &child.id().to_string()])
            .creation_flags(0x0800_0000) // CREATE_NO_WINDOW
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
    let _ = child.kill();
}

/// The test that had started but not finished: libtest prints `test name ... ` before running it.
fn unfinished_test(stdout: &str) -> Option<String> {
    // After 60 s libtest may add "test name has been running for over 60 seconds" to that same line.
    let started = Regex::new(r"^test (\S+) \.\.\. ?(.*)$").unwrap();
    stdout.lines().rev().find_map(|l| {
        let c = started.captures(l)?;
        let rest = c.get(2).map_or("", |m| m.as_str());
        let finished = ["ok", "FAILED", "ignored"].iter().any(|r| rest.starts_with(r));
        (!finished).then(|| c[1].to_string())
    })
}

/// Turn a stopped run into a result that says what happened.
fn mark_over_time(r: &mut CheckResult, stdout: &str, phase: &str, limit: Duration) {
    r.ok = false;
    let note = if phase == "build" {
        format!("The build was still going after {} minutes, so Oxide stopped it. Another cargo may be holding the build folder; try again.", limit.as_secs() / 60)
    } else {
        format!("Still running after {} seconds, so Oxide stopped it. Look for a loop that never ends, or a wait for something that never comes.", limit.as_secs())
    };
    r.raw_tail.push_str(&format!("\n{note}\n"));
    if phase == "build" {
        r.diagnostics.push(Diagnostic { level: "error".into(), message: note, rendered: String::new(), file: None, line: None });
        return;
    }
    // The tests compiled; an empty result list is from the stop, not from a compile error.
    r.compiled = true;
    r.diagnostics.retain(|d| d.message != "the tests could not run");
    let hung = unfinished_test(stdout);
    let objective = hung.as_deref().and_then(|h| r.objectives.iter_mut().find(|o| h == o.test || h.ends_with(&format!("::{}", o.test))));
    match objective {
        Some(o) => {
            o.status = ObjectiveStatus::Failed;
            o.output = Some(note);
        }
        None => r.other_failures.push(format!("{}: {note}", hung.as_deref().unwrap_or("a test"))),
    }
}

/// `cargo clippy` with no warnings at all. Only worth running after `run` is ok.
pub fn clippy_clean(workspace: &Path, target_dir: &Path) -> Result<bool> {
    let out = cargo(workspace, target_dir)
        .args(["clippy", "--quiet", "--message-format=json", "--", "-W", "clippy::all"])
        .output()
        .context("running cargo clippy")?;
    let stdout = String::from_utf8_lossy(&out.stdout);
    let any_warning = stdout.lines().filter_map(|l| serde_json::from_str::<CargoMsg>(l).ok()).any(|m| {
        m.reason == "compiler-message"
            && m.message.as_ref().is_some_and(|m| m.level == "warning" || m.level == "error")
    });
    Ok(out.status.success() && !any_warning)
}

#[derive(Deserialize)]
struct CargoMsg {
    reason: String,
    message: Option<RustcMsg>,
}
#[derive(Deserialize)]
struct RustcMsg {
    level: String,
    message: String,
    rendered: Option<String>,
    #[serde(default)]
    spans: Vec<Span>,
}
#[derive(Deserialize)]
struct Span {
    file_name: String,
    line_start: u32,
    is_primary: bool,
}

/// Pure parser, so it can be tested with captured output.
pub fn parse(stdout: &str, stderr: &str, exit_ok: bool, objectives: &[Objective]) -> CheckResult {
    let test_line = Regex::new(r"^test (\S+) \.\.\. (ok|FAILED|ignored)").unwrap();
    let mut diagnostics = Vec::new();
    let mut results: Vec<(String, bool)> = Vec::new();

    for line in stdout.lines() {
        if line.starts_with('{') {
            if let Ok(m) = serde_json::from_str::<CargoMsg>(line)
                && m.reason == "compiler-message"
                && let Some(msg) = m.message
                && (msg.level == "error" || msg.level == "warning")
            {
                let primary = msg.spans.iter().find(|s| s.is_primary);
                diagnostics.push(Diagnostic {
                    level: msg.level,
                    message: msg.message,
                    rendered: msg.rendered.unwrap_or_default(),
                    file: primary.map(|s| s.file_name.clone()),
                    line: primary.map(|s| s.line_start),
                });
            }
        } else if let Some(c) = test_line.captures(line) {
            results.push((c[1].to_string(), &c[2] == "ok"));
        }
    }

    // Compile errors cargo reports on stderr without JSON (e.g. manifest errors).
    let compile_failed = diagnostics.iter().any(|d| d.level == "error")
        || stderr.contains("error: could not compile")
        || stderr.contains("error: failed to parse manifest")
        || (!exit_ok && results.is_empty());
    if compile_failed && diagnostics.iter().all(|d| d.level != "error") {
        diagnostics.push(Diagnostic {
            level: "error".into(),
            message: "the tests could not run".into(),
            rendered: tail(stderr, 2000),
            file: None,
            line: None,
        });
    }

    let failure_outputs = parse_failure_outputs(stdout);

    let objectives: Vec<ObjectiveResult> = objectives
        .iter()
        .map(|o| {
            let hit = results.iter().find(|(name, _)| name == &o.test || name.ends_with(&format!("::{}", o.test)));
            let status = match hit {
                Some((_, true)) => ObjectiveStatus::Passed,
                Some((_, false)) => ObjectiveStatus::Failed,
                None => ObjectiveStatus::NotRun,
            };
            let output = hit
                .filter(|(_, ok)| !ok)
                .and_then(|(name, _)| failure_outputs.iter().find(|(n, _)| n == name).map(|(_, t)| t.clone()));
            ObjectiveResult { test: o.test.clone(), label: o.label.clone(), status, output }
        })
        .collect();

    let objective_names: Vec<&str> = objectives.iter().map(|o| o.test.as_str()).collect();
    let other_failures = results
        .iter()
        .filter(|(name, ok)| !ok && !objective_names.iter().any(|o| name == o || name.ends_with(&format!("::{o}"))))
        .map(|(n, _)| n.clone())
        .collect();

    let ok = !compile_failed && !objectives.is_empty() && objectives.iter().all(|o| o.status == ObjectiveStatus::Passed);

    let mut raw = String::new();
    for line in stdout.lines().filter(|l| !l.starts_with('{')) {
        raw.push_str(line);
        raw.push('\n');
    }
    raw.push_str(stderr);

    CheckResult {
        ok,
        compiled: !compile_failed,
        objectives,
        diagnostics,
        other_failures,
        raw_tail: tail(&raw, 4000),
        duration_ms: 0,
    }
}

/// `---- name stdout ----` blocks printed for failed tests.
fn parse_failure_outputs(stdout: &str) -> Vec<(String, String)> {
    let header = Regex::new(r"^---- (\S+) stdout ----$").unwrap();
    let mut out: Vec<(String, String)> = Vec::new();
    let mut current: Option<(String, String)> = None;
    for line in stdout.lines() {
        if let Some(c) = header.captures(line) {
            if let Some(cur) = current.take() {
                out.push(cur);
            }
            current = Some((c[1].to_string(), String::new()));
        } else if line.starts_with("failures:") || line.starts_with("test result:") {
            if let Some(cur) = current.take() {
                out.push(cur);
            }
        } else if let Some((_, buf)) = current.as_mut()
            && !line.starts_with("note: run with `RUST_BACKTRACE=1`")
        {
            buf.push_str(line);
            buf.push('\n');
        }
    }
    if let Some(cur) = current.take() {
        out.push(cur);
    }
    for (_, t) in out.iter_mut() {
        *t = t.trim().to_string();
    }
    out
}

fn tail(s: &str, n: usize) -> String {
    if s.len() <= n {
        return s.to_string();
    }
    let start = s.len() - n;
    let start = s.char_indices().map(|(i, _)| i).find(|&i| i >= start).unwrap_or(0);
    format!("…{}", &s[start..])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn objs(names: &[&str]) -> Vec<Objective> {
        names.iter().map(|n| Objective { test: n.to_string(), label: n.to_string() }).collect()
    }

    #[test]
    fn maps_test_lines_to_objectives() {
        let stdout = "\
running 3 tests
test tests::adds ... ok
test tests::subtracts ... FAILED
test extra ... FAILED

failures:

---- tests::subtracts stdout ----
thread 'tests::subtracts' panicked at src/lib.rs:12:9:
assertion `left == right` failed
  left: 1
 right: 2
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- extra stdout ----
boom

failures:
    tests::subtracts
    extra

test result: FAILED. 1 passed; 2 failed; 0 ignored
";
        let r = parse(stdout, "", false, &objs(&["adds", "subtracts", "missing"]));
        assert!(r.compiled);
        assert!(!r.ok);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Passed);
        assert_eq!(r.objectives[1].status, ObjectiveStatus::Failed);
        assert!(r.objectives[1].output.as_deref().unwrap().contains("left: 1"));
        assert!(!r.objectives[1].output.as_deref().unwrap().contains("RUST_BACKTRACE"));
        assert_eq!(r.objectives[2].status, ObjectiveStatus::NotRun);
        assert_eq!(r.other_failures, ["extra"]);
    }

    #[test]
    fn compile_error_is_a_diagnostic() {
        let stdout = r#"{"reason":"compiler-message","package_id":"x","manifest_path":"x","target":{},"message":{"rendered":"error[E0308]: mismatched types\n --> src/lib.rs:2:5\n","$message_type":"diagnostic","children":[],"code":null,"level":"error","message":"mismatched types","spans":[{"file_name":"src/lib.rs","line_start":2,"is_primary":true}]}}
{"reason":"build-finished","success":false}
"#;
        let r = parse(stdout, "error: could not compile `x` (lib test) due to 1 previous error\n", false, &objs(&["adds"]));
        assert!(!r.compiled);
        assert!(!r.ok);
        assert_eq!(r.diagnostics.len(), 1);
        assert_eq!(r.diagnostics[0].file.as_deref(), Some("src/lib.rs"));
        assert_eq!(r.diagnostics[0].line, Some(2));
        assert_eq!(r.objectives[0].status, ObjectiveStatus::NotRun);
    }

    #[test]
    fn cargo_refusing_to_run_is_a_failure() {
        let r = parse("", "error: current package believes it's in a workspace when it's not
", false, &objs(&["adds"]));
        assert!(!r.compiled);
        assert!(r.diagnostics[0].rendered.contains("workspace"));
    }

    #[test]
    fn all_green() {
        let r = parse("test adds ... ok\ntest result: ok. 1 passed\n", "", true, &objs(&["adds"]));
        assert!(r.ok);
    }

    #[test]
    fn unfinished_test_is_the_one_without_a_result() {
        let out = "running 3 tests\ntest tests::adds ... ok\ntest tests::spins ... ";
        assert_eq!(unfinished_test(out).as_deref(), Some("tests::spins"));
        let warned = "test tests::adds ... ok\ntest tests::spins ... test tests::spins has been running for over 60 seconds";
        assert_eq!(unfinished_test(warned).as_deref(), Some("tests::spins"));
        assert_eq!(unfinished_test("test tests::adds ... ok\n"), None);
    }

    #[test]
    fn stopped_tests_fail_the_hung_objective() {
        let out = "{\"reason\":\"build-finished\",\"success\":true}\nrunning 2 tests\ntest tests::adds ... ok\ntest tests::spins ... ";
        let mut r = parse(out, "", false, &objs(&["adds", "spins", "later"]));
        mark_over_time(&mut r, out, "tests", Duration::from_secs(60));
        assert!(r.compiled, "it built; the stop is not a compile error");
        assert!(r.diagnostics.is_empty());
        assert!(!r.ok);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Passed);
        assert_eq!(r.objectives[1].status, ObjectiveStatus::Failed);
        assert!(r.objectives[1].output.as_deref().unwrap().contains("Still running after 60 seconds"));
        assert_eq!(r.objectives[2].status, ObjectiveStatus::NotRun);
    }
}
