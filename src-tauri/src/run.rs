//! "Run" for a quest's program: `cargo run` for cargo quests (when the workspace
//! has a binary), `src/main.lua` in-process for Lua quests. Output streams to the
//! UI as `run:output` lines; `run_input` feeds stdin; `run:done` carries the exit code.

use std::io::{BufRead, BufReader, Write};
use std::path::Path;
use std::process::{Child, ChildStdin, Command, Stdio};
use std::sync::Mutex;

use oxide_core::content::{Quest, Runner};
use serde::Serialize;
use tauri::{AppHandle, Manager};

pub struct Running {
    pub quest_id: String,
    child: Child,
    stdin: Option<ChildStdin>,
}

#[derive(Serialize, Clone)]
struct OutputEvent<'a> {
    quest_id: &'a str,
    /// `stdout` | `stderr` | `build` (stderr before the program's first stdout line) | `system`
    stream: &'a str,
    line: &'a str,
}

#[derive(Serialize, Clone)]
struct DoneEvent<'a> {
    quest_id: &'a str,
    code: Option<i32>,
}

/// Whether "Run" makes sense for this workspace.
pub fn has_program(quest: &Quest, dir: &Path) -> bool {
    match quest.runner {
        Runner::Cargo => {
            dir.join("src/main.rs").is_file()
                || std::fs::read_to_string(dir.join("Cargo.toml")).is_ok_and(|t| t.contains("[[bin]]"))
        }
        Runner::Lua => dir.join("src/main.lua").is_file(),
        Runner::Gradle => false,
    }
}

fn emit_line(app: &AppHandle, quest_id: &str, stream: &str, line: &str) {
    super::emit(app, "run:output", OutputEvent { quest_id, stream, line });
}

/// Start the program; stops a previous one first.
pub fn start(
    app: &AppHandle,
    slot: &Mutex<Option<Running>>,
    quest: &Quest,
    dir: &Path,
    target_dir: &Path,
) -> Result<(), String> {
    stop(slot);
    if !has_program(quest, dir) {
        return Err("this quest has no program to run (a library with tests only)".into());
    }
    match quest.runner {
        Runner::Cargo => start_cargo(app, slot, quest, dir, target_dir),
        Runner::Lua => {
            run_lua(app, quest, dir);
            Ok(())
        }
        Runner::Gradle => Err("the gradle runner is not built yet".into()),
    }
}

fn start_cargo(
    app: &AppHandle,
    slot: &Mutex<Option<Running>>,
    quest: &Quest,
    dir: &Path,
    target_dir: &Path,
) -> Result<(), String> {
    let mut cmd = Command::new("cargo");
    cmd.args(["run", "--quiet"])
        .current_dir(dir)
        .env("CARGO_TERM_COLOR", "never")
        .env("CARGO_TARGET_DIR", target_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    let mut child = cmd.spawn().map_err(|e| format!("starting cargo run: {e}"))?;
    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdin = child.stdin.take();
    let id = quest.id.clone();
    emit_line(app, &id, "system", "$ cargo run");

    // Until the program prints its first stdout line, stderr is cargo/rustc build
    // output (warnings from `todo!()` stubs): tagged `build` so the UI can fold it.
    let program_started = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    for (stream, pipe) in [("stdout", stdout.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>)), ("stderr", stderr.map(|p| Box::new(p) as Box<dyn std::io::Read + Send>))] {
        if let Some(pipe) = pipe {
            let app = app.clone();
            let id = id.clone();
            let started = program_started.clone();
            std::thread::spawn(move || {
                for line in BufReader::new(pipe).lines().map_while(Result::ok) {
                    let tag = match stream {
                        "stdout" => {
                            started.store(true, std::sync::atomic::Ordering::Relaxed);
                            "stdout"
                        }
                        _ if !started.load(std::sync::atomic::Ordering::Relaxed) => "build",
                        _ => "stderr",
                    };
                    emit_line(&app, &id, tag, &line);
                }
            });
        }
    }
    *slot.lock().unwrap() = Some(Running { quest_id: id.clone(), child, stdin });

    // Wait for exit on its own thread; the slot is cleared when it is still ours.
    let app2 = app.clone();
    std::thread::spawn(move || {
        loop {
            std::thread::sleep(std::time::Duration::from_millis(150));
            let state = app2.state::<super::AppState>();
            let mut guard = state.running.lock().unwrap();
            let Some(r) = guard.as_mut() else { return };
            if r.quest_id != id {
                return;
            }
            match r.child.try_wait() {
                Ok(Some(status)) => {
                    *guard = None;
                    drop(guard);
                    super::emit(&app2, "run:done", DoneEvent { quest_id: &id, code: status.code() });
                    return;
                }
                Ok(None) => {}
                Err(_) => {
                    *guard = None;
                    return;
                }
            }
        }
    });
    Ok(())
}

/// Lua has no process: run `src/main.lua` in-process and emit what it printed.
fn run_lua(app: &AppHandle, quest: &Quest, dir: &Path) {
    let app = app.clone();
    let id = quest.id.clone();
    let dir = dir.to_path_buf();
    let lib = quest.lib_dir.clone();
    std::thread::spawn(move || {
        emit_line(&app, &id, "system", "$ lua src/main.lua");
        let (out, err) = oxide_core::lua::run_script(&dir, lib.as_deref(), "src/main.lua");
        for line in out.lines() {
            emit_line(&app, &id, "stdout", line);
        }
        let code = match err {
            Some(e) => {
                emit_line(&app, &id, "stderr", &e);
                Some(1)
            }
            None => Some(0),
        };
        super::emit(&app, "run:done", DoneEvent { quest_id: &id, code });
    });
}

pub fn input(slot: &Mutex<Option<Running>>, text: &str) -> Result<(), String> {
    let mut guard = slot.lock().unwrap();
    let r = guard.as_mut().ok_or("nothing is running")?;
    let stdin = r.stdin.as_mut().ok_or("the program does not read input")?;
    stdin.write_all(text.as_bytes()).and_then(|_| stdin.write_all(b"\n")).and_then(|_| stdin.flush()).map_err(|e| e.to_string())
}

pub fn stop(slot: &Mutex<Option<Running>>) {
    if let Some(mut r) = slot.lock().unwrap().take() {
        let _ = r.child.kill();
        let _ = r.child.wait();
    }
}
