use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// Write every task to the file, one per line. Replaces the old file.
pub fn save_tasks(path: &Path, tasks: &[&str]) -> Result<()> {
    // TODO: build one String with every task followed by a line break (push_str, then push('\n')),
    //       then write it to the file; on an error, add the context "could not write <path>"
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_todo_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn saves_one_line_per_task() {
        let path = temp_path("saved");
        save_tasks(&path, &["3|buy milk", "1|call mum"]).unwrap();
        let written = fs::read_to_string(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(written.unwrap(), format!("{}\n{}\n", "3|buy milk", "1|call mum"));
    }

    #[test]
    fn a_write_error_names_the_path() {
        let path = std::env::temp_dir().join("oxide_no_such_folder").join("list.txt");
        let err = save_tasks(&path, &["3|buy milk"]).unwrap_err();
        assert_eq!(err.to_string(), format!("could not write {}", path.display()));
    }
}
