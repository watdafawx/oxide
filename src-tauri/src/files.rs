//! The in-app editor's view of a quest workspace: list, read and write files
//! under `~/oxide-quests/<quest>/`, never outside it. Writes land on disk like a
//! save from any editor, so the watcher re-grades them.

use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::Serialize;

/// Folders never shown or written.
const SKIP_DIRS: &[&str] = &["target", "build", ".gradle", ".git", "node_modules", ".idea", ".vscode"];
/// Files the learner reads but must not change (the spec).
fn is_spec(rel: &str) -> bool {
    let r = rel.replace('\\', "/");
    r == "tests.lua"
        || r.starts_with("tests/")
        || r.contains("/test/")
        || r == "Cargo.lock"
        || r.ends_with(".jar")
        || r.starts_with("gradle/")
        || r == "gradlew"
        || r == "gradlew.bat"
}
const MAX_BYTES: u64 = 512 * 1024;

#[derive(Serialize)]
pub struct FileEntry {
    /// Relative path with `/` separators.
    pub path: String,
    pub bytes: u64,
    pub read_only: bool,
    /// The file the learner most likely edits first (`src/lib.rs`, `src/main.lua`, the one `src/main/java` file).
    pub primary: bool,
}

/// Every file in the workspace, sorted, specs marked read-only.
pub fn list(workspace: &Path) -> Result<Vec<FileEntry>, String> {
    let mut out = Vec::new();
    walk(workspace, workspace, &mut out)?;
    out.sort_by(|a, b| (a.read_only, &a.path).cmp(&(b.read_only, &b.path)));
    let primary = out
        .iter()
        .position(|f| !f.read_only && matches!(f.path.as_str(), "src/lib.rs" | "src/main.lua"))
        .or_else(|| out.iter().position(|f| !f.read_only && f.path.starts_with("src/main/java/") && f.path.ends_with(".java")))
        .or_else(|| out.iter().position(|f| !f.read_only && f.path.starts_with("src/") && f.path.ends_with(".lua")))
        .or_else(|| out.iter().position(|f| !f.read_only && f.path.starts_with("src/")))
        .or_else(|| out.iter().position(|f| !f.read_only));
    if let Some(i) = primary {
        out[i].primary = true;
    }
    Ok(out)
}

fn walk(root: &Path, dir: &Path, out: &mut Vec<FileEntry>) -> Result<(), String> {
    let rd = fs::read_dir(dir).map_err(|e| format!("{}: {e}", dir.display()))?;
    for entry in rd.flatten() {
        let p = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if p.is_dir() {
            if !SKIP_DIRS.contains(&name.as_str()) {
                walk(root, &p, out)?;
            }
            continue;
        }
        let Ok(meta) = entry.metadata() else { continue };
        let rel = p.strip_prefix(root).unwrap_or(&p).to_string_lossy().replace('\\', "/");
        out.push(FileEntry { read_only: is_spec(&rel) || meta.len() > MAX_BYTES, path: rel, bytes: meta.len(), primary: false });
    }
    Ok(())
}

/// `rel` resolved inside `workspace`, or an error: no `..`, no absolute paths, no skipped folders.
pub fn resolve(workspace: &Path, rel: &str) -> Result<PathBuf, String> {
    let rel_path = Path::new(rel);
    if rel_path.is_absolute() {
        return Err("absolute paths are not allowed".into());
    }
    for c in rel_path.components() {
        match c {
            Component::Normal(seg) => {
                if SKIP_DIRS.contains(&seg.to_string_lossy().as_ref()) {
                    return Err(format!("`{rel}` is outside the editable files"));
                }
            }
            Component::CurDir => {}
            _ => return Err(format!("`{rel}` leaves the workspace")),
        }
    }
    Ok(workspace.join(rel_path))
}

pub fn read(workspace: &Path, rel: &str) -> Result<String, String> {
    let p = resolve(workspace, rel)?;
    let meta = fs::metadata(&p).map_err(|e| format!("{rel}: {e}"))?;
    if meta.len() > MAX_BYTES {
        return Err(format!("{rel} is too big to edit here"));
    }
    fs::read_to_string(&p).map_err(|e| format!("{rel}: {e}"))
}

/// Write the learner's text; refuses spec files. Creates parent folders for new files.
pub fn write(workspace: &Path, rel: &str, text: &str) -> Result<(), String> {
    if is_spec(rel) {
        return Err(format!("{rel} is the spec; it is read-only"));
    }
    let p = resolve(workspace, rel)?;
    if let Some(parent) = p.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("{rel}: {e}"))?;
    }
    fs::write(&p, text).map_err(|e| format!("{rel}: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lists_marks_specs_and_primary() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();
        fs::create_dir_all(ws.join("src")).unwrap();
        fs::create_dir_all(ws.join("tests")).unwrap();
        fs::create_dir_all(ws.join("target/debug")).unwrap();
        fs::write(ws.join("Cargo.toml"), "x").unwrap();
        fs::write(ws.join("src/lib.rs"), "y").unwrap();
        fs::write(ws.join("tests/spec.rs"), "z").unwrap();
        fs::write(ws.join("target/debug/junk"), "w").unwrap();
        let files = list(ws).unwrap();
        let paths: Vec<_> = files.iter().map(|f| f.path.as_str()).collect();
        assert_eq!(paths, ["Cargo.toml", "src/lib.rs", "tests/spec.rs"]);
        assert!(files.iter().find(|f| f.path == "tests/spec.rs").unwrap().read_only);
        assert!(files.iter().find(|f| f.path == "src/lib.rs").unwrap().primary);
    }

    #[test]
    fn write_stays_inside_and_refuses_specs() {
        let tmp = tempfile::tempdir().unwrap();
        let ws = tmp.path();
        assert!(write(ws, "../evil.rs", "x").is_err());
        assert!(write(ws, "C:/evil.rs", "x").is_err());
        assert!(write(ws, "target/x.rs", "x").is_err());
        assert!(write(ws, "tests.lua", "x").is_err());
        write(ws, "src/lib.rs", "fn a() {}").unwrap();
        assert_eq!(read(ws, "src/lib.rs").unwrap(), "fn a() {}");
    }
}
