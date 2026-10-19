//! The learner's working copy of a quest: `~/oxide-quests/<world>/<quest>/`.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use anyhow::{Context, Result};

use crate::content::Quest;

/// `OXIDE_QUESTS` env var, else `~/oxide-quests`.
pub fn root() -> PathBuf {
    if let Some(p) = std::env::var_os("OXIDE_QUESTS") {
        return PathBuf::from(p);
    }
    directories::UserDirs::new()
        .map(|u| u.home_dir().join("oxide-quests"))
        .unwrap_or_else(|| PathBuf::from("oxide-quests"))
}

pub fn dir_for(root: &Path, quest_id: &str) -> PathBuf {
    let mut p = root.to_path_buf();
    for part in quest_id.split('/') {
        p.push(part);
    }
    p
}

/// Copy the starter in if the workspace does not exist yet. Idempotent.
pub fn start(root: &Path, quest: &Quest) -> Result<PathBuf> {
    let dir = dir_for(root, &quest.id);
    if !quest.runner.is_project(&dir) {
        copy_dir(&quest.starter_dir(), &dir)
            .with_context(|| format!("copying starter for {}", quest.id))?;
    }
    Ok(dir)
}

/// Throw the learner's copy away and start over.
pub fn reset(root: &Path, quest: &Quest) -> Result<PathBuf> {
    let dir = dir_for(root, &quest.id);
    if dir.exists() {
        fs::remove_dir_all(&dir).with_context(|| format!("removing {}", dir.display()))?;
    }
    start(root, quest)
}

/// Open `dir` in the learner's editor. `editor` is a command like `code`, `zed`, `cursor`.
pub fn open_in_editor(dir: &Path, editor: &str) -> Result<()> {
    let mut parts = editor.split_whitespace();
    let exe = parts.next().unwrap_or("code");
    let extra: Vec<&str> = parts.collect();
    let mut cmd = if cfg!(windows) {
        // `code` is code.cmd on Windows: needs the shell.
        let mut c = Command::new("cmd");
        c.args(["/C", exe]);
        c
    } else {
        Command::new(exe)
    };
    cmd.args(&extra).arg(dir);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x0800_0000);
    }
    cmd.spawn().with_context(|| format!("launching editor `{editor}`"))?;
    Ok(())
}

/// Recursive copy, skipping `target/` and lock files. Copied files get a fresh
/// mtime: `fs::copy` keeps the source's on Windows, and cargo would then reuse a
/// build made from the previous contents of the same path.
pub fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if name_str == "target" || name_str == "Cargo.lock" {
            continue;
        }
        let from = entry.path();
        let to = dst.join(&name);
        if from.is_dir() {
            copy_dir(&from, &to)?;
        } else {
            fs::copy(&from, &to)?;
            fs::File::options().write(true).open(&to)?.set_modified(std::time::SystemTime::now())?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn copy_skips_target() {
        let tmp = tempfile::tempdir().unwrap();
        let src = tmp.path().join("src");
        fs::create_dir_all(src.join("target/debug")).unwrap();
        fs::create_dir_all(src.join("src")).unwrap();
        fs::write(src.join("Cargo.toml"), "x").unwrap();
        fs::write(src.join("src/lib.rs"), "y").unwrap();
        fs::write(src.join("target/debug/junk"), "z").unwrap();
        let dst = tmp.path().join("dst");
        copy_dir(&src, &dst).unwrap();
        assert!(dst.join("src/lib.rs").is_file());
        assert!(!dst.join("target").exists());
    }

    #[test]
    fn dir_for_nests_by_id() {
        let d = dir_for(Path::new("root"), "01-foundations/02-vars");
        assert_eq!(d, Path::new("root").join("01-foundations").join("02-vars"));
    }
}
