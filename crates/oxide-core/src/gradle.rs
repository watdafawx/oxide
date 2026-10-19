//! The gradle runner: `gradlew test` in the workspace, objectives from the JUnit XML reports.
//!
//! Compiler errors come from stdout/stderr (`File.java:12: error: ...`), test results from
//! `build/test-results/test/TEST-*.xml`. Gradle builds in-tree (`build/`, `.gradle/`) and
//! keeps its dependency cache in a short shared `GRADLE_USER_HOME`.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use anyhow::{Context, Result, bail};
use regex::Regex;

use crate::content::Objective;
use crate::grader::{CheckResult, Diagnostic, ObjectiveResult, ObjectiveStatus};

/// The JDK a quest is graded with when neither track nor quest says otherwise.
pub const DEFAULT_JDK: u32 = 21;

const TIMEOUT: Duration = Duration::from_secs(10 * 60);

/// Where Gradle keeps its distribution and dependency cache: next to the shared target dir.
pub fn user_home_for(target_dir: &Path) -> PathBuf {
    target_dir.parent().unwrap_or(target_dir).join(".gradle")
}

/// Find a JDK of major version `major`: `OXIDE_JDK_<n>`, then Adoptium's install folder,
/// then `JAVA_HOME` if its `release` file says that major.
pub fn find_jdk(major: u32) -> Result<PathBuf> {
    if let Some(p) = std::env::var_os(format!("OXIDE_JDK_{major}")).map(PathBuf::from)
        && p.is_dir()
    {
        return Ok(p);
    }
    let prefix = format!("jdk-{major}");
    let adoptium = Path::new(r"C:\Program Files\Eclipse Adoptium");
    if let Ok(rd) = std::fs::read_dir(adoptium) {
        let mut found: Vec<PathBuf> = rd
            .flatten()
            .filter(|e| {
                let n = e.file_name().to_string_lossy().to_string();
                // `jdk-21.0.12...` but not `jdk-210`.
                n.strip_prefix(&prefix).is_some_and(|rest| rest.is_empty() || !rest.starts_with(|c: char| c.is_ascii_digit()))
            })
            .map(|e| e.path())
            .filter(|p| p.join("bin").is_dir())
            .collect();
        found.sort();
        if let Some(p) = found.pop() {
            return Ok(p);
        }
    }
    if let Some(home) = std::env::var_os("JAVA_HOME").map(PathBuf::from)
        && let Ok(release) = std::fs::read_to_string(home.join("release"))
        && release_major(&release) == Some(major)
    {
        return Ok(home);
    }
    bail!("JDK {major} not found: install Temurin {major}")
}

/// Major version from a JDK `release` file (`JAVA_VERSION="21.0.12"`, `"1.8.0_402"` = 8).
fn release_major(release: &str) -> Option<u32> {
    let v = release.lines().find_map(|l| l.strip_prefix("JAVA_VERSION="))?.trim().trim_matches('"');
    let mut parts = v.split(['.', '_', '-', '+']);
    let first: u32 = parts.next()?.parse().ok()?;
    if first == 1 { parts.next()?.parse().ok() } else { Some(first) }
}

struct Output {
    stdout: String,
    stderr: String,
    ok: bool,
}

fn gradlew(workspace: &Path, jdk_home: &Path, user_home: &Path, offline: bool) -> Result<Output> {
    let script = workspace.join(if cfg!(windows) { "gradlew.bat" } else { "gradlew" });
    if !script.is_file() {
        bail!("{} is missing: the quest needs the Gradle wrapper", script.display());
    }
    let mut c = Command::new(&script);
    c.current_dir(workspace)
        .args(["test", "--console=plain"])
        .env("JAVA_HOME", jdk_home)
        .env("GRADLE_USER_HOME", user_home)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if offline {
        c.arg("--offline");
    }
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        c.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
    }
    let mut child = c.spawn().with_context(|| format!("starting {}", script.display()))?;
    // The Gradle daemon inherits our pipes and outlives the build, so the readers never see EOF:
    // collect into shared buffers and stop waiting shortly after the client exits.
    type Buf = std::sync::Arc<std::sync::Mutex<Vec<u8>>>;
    let drain = |mut r: Box<dyn Read + Send>| -> (Buf, std::thread::JoinHandle<()>) {
        let buf: Buf = Default::default();
        let sink = buf.clone();
        let h = std::thread::spawn(move || {
            let mut chunk = [0u8; 8192];
            while let Ok(n) = r.read(&mut chunk) {
                if n == 0 {
                    break;
                }
                sink.lock().unwrap().extend_from_slice(&chunk[..n]);
            }
        });
        (buf, h)
    };
    let (out_buf, out_h) = drain(Box::new(child.stdout.take().unwrap()));
    let (err_buf, err_h) = drain(Box::new(child.stderr.take().unwrap()));
    let start = Instant::now();
    let status = loop {
        if let Some(s) = child.try_wait()? {
            break s;
        }
        if start.elapsed() > TIMEOUT {
            let _ = child.kill();
            bail!("gradle took longer than {} minutes and was stopped", TIMEOUT.as_secs() / 60);
        }
        std::thread::sleep(Duration::from_millis(100));
    };
    let grace = Instant::now();
    while !(out_h.is_finished() && err_h.is_finished()) && grace.elapsed() < Duration::from_millis(1500) {
        std::thread::sleep(Duration::from_millis(20));
    }
    let text = |b: &Buf| String::from_utf8_lossy(&b.lock().unwrap()).into_owned();
    Ok(Output { stdout: text(&out_buf), stderr: text(&err_buf), ok: status.success() })
}

fn is_resolution_error(o: &Output) -> bool {
    let all = format!("{}\n{}", o.stdout, o.stderr);
    all.contains("Could not resolve") || all.contains("No cached version") || all.contains("offline mode")
}

/// Run the tests in `workspace` with a JDK of major version `jdk` (default 21) and grade `objectives`.
pub fn run(workspace: &Path, jdk: Option<u32>, user_home: &Path, objectives: &[Objective]) -> Result<CheckResult> {
    let start = Instant::now();
    let jdk_home = find_jdk(jdk.unwrap_or(DEFAULT_JDK))?;
    std::fs::create_dir_all(user_home).with_context(|| format!("creating {}", user_home.display()))?;
    // Stale reports from an earlier run must not grade this one.
    let results_dir = workspace.join("build/test-results/test");
    let _ = std::fs::remove_dir_all(&results_dir);

    // Offline is the fast path once something is cached; the first run has to resolve online.
    let offline = user_home.join("caches/modules-2").is_dir();
    let mut out = gradlew(workspace, &jdk_home, user_home, offline)?;
    if !out.ok && offline && is_resolution_error(&out) {
        out = gradlew(workspace, &jdk_home, user_home, false)?;
    }

    let mut xmls = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&results_dir) {
        let mut paths: Vec<PathBuf> = rd
            .flatten()
            .map(|e| e.path())
            .filter(|p| p.extension().is_some_and(|e| e == "xml"))
            .collect();
        paths.sort();
        for p in paths {
            if let Ok(t) = std::fs::read_to_string(&p) {
                xmls.push(t);
            }
        }
    }
    let mut r = parse(&out.stdout, &out.stderr, &xmls, out.ok, objectives);
    let root = workspace.to_string_lossy().replace('\\', "/");
    let root = format!("{}/", root.trim_end_matches('/'));
    for d in &mut r.diagnostics {
        if let Some(f) = d.file.as_mut()
            && let Some(rest) = f.strip_prefix(&root)
        {
            *f = rest.to_string();
        }
        d.rendered = d.rendered.replace('\\', "/").replace(&root, "");
    }
    r.duration_ms = start.elapsed().as_millis() as u64;
    Ok(r)
}

#[derive(Debug)]
struct Case {
    /// Method name without parameters: `greets()` → `greets`.
    method: String,
    /// `pkg.Class.method`, nested classes with dots.
    full: String,
    outcome: Outcome,
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Passed,
    Failed(String),
    Skipped,
}

fn parse_cases(xml_texts: &[String]) -> Vec<Case> {
    let mut cases = Vec::new();
    for text in xml_texts {
        let Ok(doc) = roxmltree::Document::parse_with_options(
            text,
            roxmltree::ParsingOptions { allow_dtd: true, ..Default::default() },
        ) else {
            continue;
        };
        for tc in doc.descendants().filter(|n| n.has_tag_name("testcase")) {
            let name = tc.attribute("name").unwrap_or("");
            let method = name.split('(').next().unwrap_or(name).trim().to_string();
            let class = tc.attribute("classname").unwrap_or("").replace('$', ".");
            let full = if class.is_empty() { method.clone() } else { format!("{class}.{method}") };
            let bad = tc.children().find(|c| c.has_tag_name("failure") || c.has_tag_name("error"));
            let outcome = if let Some(b) = bad {
                let message = b.attribute("message").unwrap_or("").trim();
                let body = b.text().unwrap_or("").trim();
                // The body holds the message plus the stack trace.
                let text = if body.is_empty() { message } else { body };
                Outcome::Failed(text.to_string())
            } else if tc.children().any(|c| c.has_tag_name("skipped")) {
                Outcome::Skipped
            } else {
                Outcome::Passed
            };
            cases.push(Case { method, full, outcome });
        }
    }
    cases
}

fn matches(case: &Case, test: &str) -> bool {
    case.method == test || case.full == test || case.full.ends_with(&format!(".{test}"))
}

/// Pure parser, so it can be tested with captured output.
pub fn parse(stdout: &str, stderr: &str, xml_texts: &[String], exit_ok: bool, objectives: &[Objective]) -> CheckResult {
    let cases = parse_cases(xml_texts);
    let mut diagnostics = parse_diagnostics(stdout, stderr);

    let compile_failed = diagnostics.iter().any(|d| d.level == "error") || (!exit_ok && cases.is_empty());
    if compile_failed && diagnostics.iter().all(|d| d.level != "error") {
        diagnostics.push(Diagnostic {
            level: "error".into(),
            message: "the tests could not run".into(),
            rendered: what_went_wrong(stdout, stderr).unwrap_or_else(|| tail(&format!("{stdout}\n{stderr}"), 2000)),
            file: None,
            line: None,
        });
    }

    let objective_results: Vec<ObjectiveResult> = objectives
        .iter()
        .map(|o| {
            let hits: Vec<&Case> = cases.iter().filter(|c| matches(c, &o.test)).collect();
            let failed = hits.iter().find_map(|c| match &c.outcome {
                Outcome::Failed(t) => Some(t.clone()),
                _ => None,
            });
            let status = if hits.is_empty() || hits.iter().any(|c| c.outcome == Outcome::Skipped) {
                ObjectiveStatus::NotRun
            } else if failed.is_some() {
                ObjectiveStatus::Failed
            } else {
                ObjectiveStatus::Passed
            };
            ObjectiveResult { test: o.test.clone(), label: o.label.clone(), status, output: failed }
        })
        .collect();

    let mut other_failures: Vec<String> = Vec::new();
    for c in &cases {
        if matches!(c.outcome, Outcome::Failed(_))
            && !objectives.iter().any(|o| matches(c, &o.test))
            && !other_failures.contains(&c.method)
        {
            other_failures.push(c.method.clone());
        }
    }

    let ok = !compile_failed
        && !objective_results.is_empty()
        && objective_results.iter().all(|o| o.status == ObjectiveStatus::Passed);

    let raw = format!("{stdout}\n{stderr}");
    CheckResult {
        ok,
        compiled: !compile_failed,
        objectives: objective_results,
        diagnostics,
        other_failures,
        raw_tail: tail(raw.trim(), 4000),
        duration_ms: 0,
    }
}

/// `path.java:LINE: error: msg` plus the source/caret/`symbol:` lines javac prints after it.
fn parse_diagnostics(stdout: &str, stderr: &str) -> Vec<Diagnostic> {
    let head = Regex::new(r"^(.+\.(?:java|kt)):(\d+): (error|warning): (.*)$").unwrap();
    let mut out: Vec<Diagnostic> = Vec::new();
    let text = format!("{stdout}\n{stderr}");
    let lines: Vec<&str> = text.lines().map(|l| l.trim_end_matches('\r')).collect();
    let mut i = 0;
    while i < lines.len() {
        let Some(c) = head.captures(lines[i]) else {
            i += 1;
            continue;
        };
        let mut rendered = lines[i].to_string();
        i += 1;
        while i < lines.len() {
            let l = lines[i];
            let ends = l.trim().is_empty()
                || head.is_match(l)
                || l.starts_with("> Task")
                || l.starts_with("FAILURE:")
                || l.starts_with("* ")
                || l.starts_with("Note:")
                || l.ends_with(" error")
                || l.ends_with(" errors")
                || l.ends_with(" warning")
                || l.ends_with(" warnings");
            if ends {
                break;
            }
            rendered.push('\n');
            rendered.push_str(l);
            i += 1;
        }
        let file = c[1].replace('\\', "/");
        let line: u32 = c[2].parse().unwrap_or(0);
        // The same diagnostic shows up once per compile task; keep the first.
        if out.iter().any(|d| d.file.as_deref() == Some(&file) && d.line == Some(line) && d.message == c[4]) {
            continue;
        }
        out.push(Diagnostic { level: c[3].to_string(), message: c[4].to_string(), rendered, file: Some(file), line: Some(line) });
    }
    out
}

/// The "* What went wrong:" section of a failed Gradle build.
fn what_went_wrong(stdout: &str, stderr: &str) -> Option<String> {
    let text = format!("{stdout}\n{stderr}");
    let (_, rest) = text.split_once("* What went wrong:")?;
    let section = rest.split("\n* Try:").next().unwrap_or(rest).trim();
    (!section.is_empty()).then(|| tail(section, 2000))
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

    fn report(cases: &str) -> String {
        format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<testsuite name=\"oxide.hello.GreetTest\" tests=\"2\" skipped=\"0\" failures=\"0\" errors=\"0\">\n<properties/>\n{cases}\n<system-out><![CDATA[]]></system-out>\n</testsuite>\n"
        )
    }

    const PASS: &str = r#"<testcase name="greets()" classname="oxide.hello.GreetTest" time="0.012"/>
<testcase name="shouts()" classname="oxide.hello.GreetTest" time="0.001"/>"#;

    #[test]
    fn passing_run() {
        let r = parse(
            "> Task :test\n\nBUILD SUCCESSFUL in 3s\n",
            "",
            &[report(PASS)],
            true,
            &objs(&["greets", "GreetTest.shouts"]),
        );
        assert!(r.ok && r.compiled);
        assert!(r.objectives.iter().all(|o| o.status == ObjectiveStatus::Passed));
        assert!(r.other_failures.is_empty());
    }

    #[test]
    fn failing_assertion_keeps_assertj_text() {
        let failing = r#"<testcase name="greets()" classname="oxide.hello.GreetTest" time="0.02">
<failure message="java.lang.AssertionError: &#10;Expecting:&#10; &lt;null&gt;&#10;to be equal to:&#10; &lt;&quot;Hello, Ada!&quot;&gt;" type="java.lang.AssertionError">java.lang.AssertionError:
Expecting:
 &lt;null&gt;
to be equal to:
 &lt;"Hello, Ada!"&gt;
	at oxide.hello.GreetTest.greets(GreetTest.java:11)
</failure></testcase>
<testcase name="extra()" classname="oxide.hello.GreetTest"><error message="boom" type="java.lang.IllegalStateException">java.lang.IllegalStateException: boom</error></testcase>
<testcase name="shouts()" classname="oxide.hello.GreetTest"/>"#;
        let r = parse("", "", &[report(failing)], false, &objs(&["greets", "shouts"]));
        assert!(r.compiled);
        assert!(!r.ok);
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Failed);
        let out = r.objectives[0].output.as_deref().unwrap();
        assert!(out.contains("Expecting:") && out.contains("Hello, Ada!") && out.contains("GreetTest.java:11"), "{out}");
        assert_eq!(r.objectives[1].status, ObjectiveStatus::Passed);
        assert_eq!(r.other_failures, ["extra"]);
    }

    #[test]
    fn compile_error_becomes_a_diagnostic() {
        let stdout = "> Task :compileJava FAILED\n\
C:\\Users\\a\\q\\src\\main\\java\\oxide\\hello\\Hello.java:5: error: incompatible types: int cannot be converted to String\n\
        return 42;\n\
               ^\n\
1 error\n\
\n\
FAILURE: Build failed with an exception.\n";
        let r = parse(stdout, "", &[], false, &objs(&["greets"]));
        assert!(!r.compiled && !r.ok);
        assert_eq!(r.diagnostics.len(), 1);
        let d = &r.diagnostics[0];
        assert_eq!(d.level, "error");
        assert_eq!(d.line, Some(5));
        assert_eq!(d.message, "incompatible types: int cannot be converted to String");
        assert!(d.file.as_deref().unwrap().ends_with("src/main/java/oxide/hello/Hello.java"));
        assert!(d.rendered.contains("return 42;") && d.rendered.contains('^'));
        assert!(!d.rendered.contains("1 error"));
        assert_eq!(r.objectives[0].status, ObjectiveStatus::NotRun);
    }

    #[test]
    fn missing_test_is_not_run() {
        let r = parse("", "", &[report(PASS)], true, &objs(&["greets", "vanished"]));
        assert!(r.compiled);
        assert!(!r.ok);
        assert_eq!(r.objectives[1].status, ObjectiveStatus::NotRun);
    }

    #[test]
    fn parameterized_and_nested_cases() {
        let xml = r#"<testcase name="fizz(int, String)[1]" classname="oxide.M$Inner"/>
<testcase name="fizz(int, String)[2]" classname="oxide.M$Inner"><failure message="x">x</failure></testcase>"#;
        let r = parse("", "", &[report(xml)], false, &objs(&["fizz", "Inner.fizz"]));
        assert_eq!(r.objectives[0].status, ObjectiveStatus::Failed);
        assert_eq!(r.objectives[1].status, ObjectiveStatus::Failed);
    }

    #[test]
    fn build_script_failure_without_reports() {
        let stderr = "FAILURE: Build failed with an exception.\n\n* What went wrong:\nCould not resolve org.assertj:assertj-core:3.27.7.\n\n* Try:\n> Run with --stacktrace\n";
        let r = parse("", stderr, &[], false, &objs(&["greets"]));
        assert!(!r.compiled);
        assert!(r.diagnostics[0].rendered.contains("Could not resolve"));
        assert!(!r.diagnostics[0].rendered.contains("Run with"));
    }

    #[test]
    fn release_file_major() {
        assert_eq!(release_major("JAVA_VERSION=\"21.0.12\"\nOS_NAME=\"Windows\""), Some(21));
        assert_eq!(release_major("JAVA_VERSION=\"17.0.20\""), Some(17));
        assert_eq!(release_major("JAVA_VERSION=\"1.8.0_402\""), Some(8));
        assert_eq!(release_major("nothing"), None);
    }
}
