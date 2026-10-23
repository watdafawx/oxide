//! Whole engine against the real tutorial content: workspace → grader → progress.
//! Needs `cargo` on PATH; takes a few seconds the first time (compiles the quest).

use std::collections::HashSet;

use chrono::Local;
use oxide_core::grader::{self, ObjectiveStatus};
use oxide_core::progress::Progress;
use oxide_core::{content, workspace};

#[test]
fn tutorial_hello_red_then_green() {
    // Own copy of the tutorial world: other worlds may be half-written while authoring.
    let tmp = tempfile::tempdir().unwrap();
    let content_dir = tmp.path().join("content");
    workspace::copy_dir(
        &oxide_core::repo_content_dir().join("tracks/rust/worlds/00-tutorial"),
        &content_dir.join("tracks/rust/worlds/00-tutorial"),
    )
    .unwrap();
    std::fs::copy(
        oxide_core::repo_content_dir().join("tracks/rust/track.toml"),
        content_dir.join("tracks/rust/track.toml"),
    )
    .unwrap();
    let catalog = content::load(&content_dir).expect("catalog loads");
    let quest = catalog.quest("rust/00-tutorial/01-hello").expect("tutorial quest exists");

    let root = tmp.path().join("quests");
    let root = root.as_path();
    let target = grader::shared_target_dir(root);
    let progress = Progress::in_memory().unwrap();

    // Start: starter copied, run records a start.
    let dir = workspace::start(root, quest).unwrap();
    progress.start_quest(&quest.id, Local::now()).unwrap();
    assert!(dir.join("src/lib.rs").is_file());

    // Red: starter compiles, objective fails at runtime with the todo!() panic.
    let red = grader::run_quest(quest, &dir, &target).unwrap();
    assert!(red.compiled, "starter must compile: {:?}", red.diagnostics);
    assert!(!red.ok);
    assert_eq!(red.objectives[0].status, ObjectiveStatus::Failed);
    assert!(red.objectives[0].output.as_deref().unwrap_or("").contains("not yet implemented"));
    progress.record_attempt(&quest.id, Local::now()).unwrap();

    // Green: drop the solution in, like the learner saving the right code.
    workspace::copy_dir(&quest.solution_dir(), &dir).unwrap();
    let green = grader::run_quest(quest, &dir, &target).unwrap();
    assert!(green.ok, "{}", green.raw_tail);
    let clean = grader::clippy_clean(&dir, &target).unwrap();
    assert!(clean);

    let c = progress.complete_quest(&catalog, quest, Local::now(), false, clean).unwrap();
    assert!(c.first_time);
    assert_eq!(c.xp_awarded, 33); // 30 * 1.1
    assert!(c.new_achievements.contains(&"first-green".to_string()));

    let profile = progress.profile().unwrap();
    assert_eq!(profile.xp, 33);
    assert_eq!(profile.streak_days, 1);
    let run = progress.run(&quest.id).unwrap().unwrap();
    assert_eq!(run.attempts, 1);

    // The next quest unlocks.
    let done: HashSet<String> = profile.completed.iter().cloned().collect();
    let next = catalog.available(&done);
    assert_eq!(next[0].id, "rust/00-tutorial/02-fix-it");
}

/// A test that never ends is stopped after the time limit and reported, instead of the check spinning forever.
#[test]
fn hanging_test_is_stopped_and_named() {
    use oxide_core::content::Objective;
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("spin");
    std::fs::create_dir_all(dir.join("src")).unwrap();
    std::fs::write(dir.join("Cargo.toml"), "[package]\nname = \"spin\"\nversion = \"0.1.0\"\nedition = \"2024\"\n\n[workspace]\n").unwrap();
    std::fs::write(
        dir.join("src/lib.rs"),
        "#[cfg(test)]\nmod tests {\n    #[test]\n    fn a_adds() { assert_eq!(1 + 1, 2); }\n    #[test]\n    fn b_spins() { loop { std::thread::sleep(std::time::Duration::from_millis(50)); } }\n}\n",
    )
    .unwrap();
    let objectives: Vec<Objective> = ["a_adds", "b_spins"].iter().map(|t| Objective { test: t.to_string(), label: t.to_string() }).collect();
    let target = tmp.path().join("target");
    let started = std::time::Instant::now();
    let r = grader::run_with_limits(&dir, &target, &objectives, std::time::Duration::from_secs(600), std::time::Duration::from_secs(3)).unwrap();
    assert!(started.elapsed() < std::time::Duration::from_secs(120), "the check came back");
    assert!(r.compiled && !r.ok);
    assert_eq!(r.objectives[0].status, ObjectiveStatus::Passed);
    assert_eq!(r.objectives[1].status, ObjectiveStatus::Failed);
    assert!(r.objectives[1].output.as_deref().unwrap().contains("Still running after 3 seconds"), "{:?}", r.objectives[1].output);
}
