//! Oxide core: everything the app does that is not a window.
//!
//! - [`content`]: load the quest catalog from `content/worlds`.
//! - [`workspace`]: the learner's copy of a quest and the editor that opens it.
//! - [`grader`]: run `cargo test` in a workspace and map tests to objectives.
//! - [`lua`]: the same for Lua quests, in an embedded Lua 5.2.
//! - [`watcher`]: re-grade when a file in the workspace is saved.
//! - [`progress`]: XP, levels, streaks, achievements in SQLite.

pub mod content;
pub mod gradle;
pub mod grader;
pub mod lua;
pub mod progress;
pub mod watcher;
pub mod workspace;

pub use content::{Catalog, Quest, QuestKind, Runner, Track, World};
pub use grader::CheckResult;
pub use progress::{Completion, Profile, Progress};

use std::path::PathBuf;

/// The `content/` directory of this repository, for `cargo run` and tests.
pub fn repo_content_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../content")
        .canonicalize()
        .unwrap_or_else(|_| PathBuf::from("content"))
}
