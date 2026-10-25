use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Write every mod to the file, one per line. Replaces the old file.
pub fn save_mods(path: &Path, mods: &[&str]) -> Result<()> {
    // TODO: build one String with every mod followed by a line break (push_str, then push('\n')),
    //       then write it to the file; on an error, add the context "could not write <path>"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_mods_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn saves_one_line_per_mod() {
        let path = temp_path("saved");
        save_mods(&path, &["sodium@5 fabric", "lithium@3 fabric"]).unwrap();
        let written = fs::read_to_string(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(written.unwrap(), format!("{}\n{}\n", "sodium@5 fabric", "lithium@3 fabric"));
    }

    #[test]
    fn a_write_error_names_the_path() {
        let path = std::env::temp_dir().join("oxide_no_such_folder").join("list.txt");
        let err = save_mods(&path, &["sodium@5 fabric"]).unwrap_err();
        assert_eq!(err.to_string(), format!("could not write {}", path.display()));
    }
}
