use std::fs;
use std::io;
use std::path::Path;

use anyhow::{Context, Result};

/// The file's text, or "" when the file does not exist yet. Other errors still fail.
pub fn load_or_empty(path: &Path) -> Result<String> {
    match fs::read_to_string(path) {
        Ok(text) => Ok(text),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(String::new()),
        Err(e) => Err(e).with_context(|| format!("could not read {}", path.display())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_mods_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn a_missing_manifest_is_an_empty_list() {
        let path = temp_path("first_start");
        let _ = fs::remove_file(&path);
        assert_eq!(load_or_empty(&path).unwrap(), "", "no file yet is an empty list, not an error");
    }

    #[test]
    fn an_existing_manifest_is_read() {
        let path = temp_path("existing");
        fs::write(&path, "sodium@5 fabric\n").unwrap();
        let loaded = load_or_empty(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(loaded.unwrap(), "sodium@5 fabric\n");
    }

    #[test]
    fn other_errors_still_fail() {
        let folder = std::env::temp_dir();
        let err = load_or_empty(&folder).expect_err("a folder is not a file, so this must stay an error");
        assert_eq!(err.to_string(), format!("could not read {}", folder.display()));
    }
}
