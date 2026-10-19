//! `cargo run -p content-check [-- --fast] [quest-id-prefix...]`
//!
//! For every quest (or those whose id starts with a given prefix):
//! - quest.toml valid, lesson present, requires resolve (via `oxide_core::content::load`)
//! - starter compiles and every objective test exists (and fails or passes; it must *run*)
//! - solution passes every objective
//!
//! `--fast` skips the cargo runs and only checks the catalog.

use std::path::PathBuf;
use std::process::ExitCode;

use oxide_core::content::{QuestKind, Runner};
use oxide_core::grader::{self, ObjectiveStatus};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let fast = args.iter().any(|a| a == "--fast");
    let prefixes: Vec<&String> = args.iter().filter(|a| !a.starts_with("--")).collect();

    let content_dir = std::env::var_os("OXIDE_CONTENT")
        .map(PathBuf::from)
        .unwrap_or_else(oxide_core::repo_content_dir);
    let catalog = match oxide_core::content::load(&content_dir) {
        Ok(c) => c,
        Err(problems) => {
            eprintln!("catalog has {} problem(s):\n{problems}", problems.0.len());
            return ExitCode::FAILURE;
        }
    };
    // A prefix matches the full id (`rust/01-foundations`) or the id inside its track (`01-foundations`).
    let quests: Vec<_> = catalog
        .quests()
        .filter(|q| {
            prefixes.is_empty()
                || prefixes.iter().any(|p| q.id.starts_with(p.as_str()) || q.local_id().starts_with(p.as_str()))
        })
        .collect();
    println!(
        "catalog ok: {} tracks, {} worlds, {} quests, {} XP total ({} selected)",
        catalog.tracks.len(),
        catalog.worlds.len(),
        catalog.quests().count(),
        catalog.total_xp(),
        quests.len()
    );
    if fast {
        return ExitCode::SUCCESS;
    }

    // Cargo keys path packages by name+version only, so starter and solution (same name)
    // would share one stale binary in a single target dir. Two dirs, deps still shared per dir.
    let target_root = std::env::temp_dir().join("oxide-content-check-target");
    let target_starter = target_root.join("starter");
    let target_solution = target_root.join("solution");

    // Same reason: every quest's package name must be unique across the catalog,
    // since the learner's quests share one target dir.
    let mut names: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let mut failures = 0;
    for q in &quests {
        if q.kind == QuestKind::Read || q.runner != Runner::Cargo {
            continue;
        }
        let starter_name = package_name(&q.starter_dir());
        let solution_name = package_name(&q.solution_dir());
        if starter_name != solution_name {
            println!("  FAIL  {}
        starter and solution package names differ", q.id);
            failures += 1;
        }
        if let Some(name) = starter_name
            && let Some(other) = names.insert(name.clone(), q.id.clone())
        {
            println!("  FAIL  {}
        package name `{name}` already used by {other}", q.id);
            failures += 1;
        }
    }
    for q in quests {
        if q.kind == QuestKind::Read {
            println!("  read  {}", q.id);
            continue;
        }
        if q.runner == Runner::Gradle {
            println!("  skip  {} (gradle runner not built yet)", q.id);
            continue;
        }
        // The plain quest, then each flavour (own starter/solution, same tests).
        let mut versions = vec![q.resolved("plain")];
        versions.extend(q.flavours().map(|f| q.resolved(f)));
        for q in &versions {
        let label = if q.flavour == "plain" { q.id.clone() } else { format!("{} [{}]", q.id, q.flavour) };
        let mut bad = Vec::new();

        match grader::run_quest(q, &q.starter_dir(), &target_starter) {
            Ok(r) => {
                if q.starter_fails_to_compile {
                    if r.compiled {
                        bad.push("starter compiles but quest.toml says starter_fails_to_compile".into());
                    }
                } else if !r.compiled {
                    bad.push("starter does not compile".to_string());
                    for d in r.diagnostics.iter().filter(|d| d.level == "error").take(3) {
                        bad.push(format!("    {}", d.rendered.lines().next().unwrap_or("")));
                    }
                } else {
                    for o in &r.objectives {
                        if o.status == ObjectiveStatus::NotRun {
                            bad.push(format!("starter: objective test `{}` not found", o.test));
                        }
                    }
                    if r.ok {
                        bad.push("starter already passes every objective".into());
                    }
                }
            }
            Err(e) => bad.push(format!("starter: {e:#}")),
        }

        match grader::run_quest(q, &q.solution_dir(), &target_solution) {
            Ok(r) => {
                if !r.ok {
                    bad.push("solution does not pass".into());
                    for d in r.diagnostics.iter().filter(|d| d.level == "error").take(3) {
                        bad.push(format!("    {}", d.rendered.lines().next().unwrap_or("")));
                    }
                    for o in r.objectives.iter().filter(|o| o.status != ObjectiveStatus::Passed) {
                        bad.push(format!("    objective `{}`: {:?}", o.test, o.status));
                    }
                    for f in &r.other_failures {
                        bad.push(format!("    extra test failed: {f}"));
                    }
                    for l in r.raw_tail.lines().rev().take(12).collect::<Vec<_>>().into_iter().rev() {
                        bad.push(format!("    | {l}"));
                    }
                }
            }
            Err(e) => bad.push(format!("solution: {e:#}")),
        }

        if bad.is_empty() {
            println!("  ok    {label}");
        } else {
            failures += 1;
            println!("  FAIL  {label}");
            for b in bad {
                println!("        {b}");
            }
        }
        }
    }

    if failures > 0 {
        eprintln!("{failures} quest(s) failed");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

/// `name = "..."` from a Cargo.toml, no toml parser needed.
fn package_name(dir: &std::path::Path) -> Option<String> {
    let text = std::fs::read_to_string(dir.join("Cargo.toml")).ok()?;
    text.lines()
        .map(str::trim)
        .find_map(|l| l.strip_prefix("name = \"").and_then(|r| r.strip_suffix('"')).map(str::to_string))
}
