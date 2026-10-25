use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

/// The whole file as text. Any error says which file: "could not read <path>".
pub fn load_text(path: &Path) -> Result<String> {
    fs::read_to_string(path).with_context(|| format!("could not read {}", path.display()))
}

/// How many tasks are in the file: one per line, empty lines don't count.
pub fn count_tasks(path: &Path) -> Result<u32> {
    let text = load_text(path)?;
    let mut count = 0;
    for line in text.lines() {
        if !line.trim().is_empty() {
            count += 1;
        }
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!("oxide_todo_{}_{}.txt", std::process::id(), name))
    }

    #[test]
    fn counts_the_tasks_in_a_file() {
        let path = temp_path("three");
        fs::write(&path, "3|buy milk\n\n1|call mum\n2|water plants\n").unwrap();
        let counted = count_tasks(&path);
        let _ = fs::remove_file(&path);
        assert_eq!(counted.unwrap(), 3, "three lines with text, one empty line");
    }

    #[test]
    fn a_missing_file_names_its_path() {
        let path = temp_path("does_not_exist");
        let _ = fs::remove_file(&path);
        let err = load_text(&path).unwrap_err();
        assert_eq!(err.to_string(), format!("could not read {}", path.display()));
    }
}
