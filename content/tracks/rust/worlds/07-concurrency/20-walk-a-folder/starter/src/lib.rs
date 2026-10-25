use std::fs;
use std::path::{Path, PathBuf};

/// Every file (not folder) below `dir`, at any depth, sorted by path.
/// A folder that cannot be read gives an empty list.
pub fn walk(dir: &Path) -> Vec<PathBuf> {
    // TODO: fs::read_dir(dir), or an empty Vec if it fails; for each entry that is Ok,
    // push a file's path, or call walk on a folder (is_dir) and push what it returns;
    // sort before returning
    let _ = dir;
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Makes a fresh folder tree under the system temp folder; returns it and its files, sorted.
    fn make_tree(tag: &str) -> (PathBuf, Vec<PathBuf>) {
        let dir = std::env::temp_dir().join(format!("oxide_conc_walk_{tag}_{}", std::process::id()));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(dir.join("photos/2024")).unwrap();
        fs::create_dir_all(dir.join("empty")).unwrap();
        let mut files = Vec::new();
        for rel in ["report.pdf", "notes.txt", "photos/beach.jpg", "photos/2024/cat.jpg", "photos/2024/dog.jpg"] {
            let path = dir.join(rel);
            fs::write(&path, rel).unwrap();
            files.push(path);
        }
        files.sort();
        (dir, files)
    }

    #[test]
    fn walk_lists_every_file_sorted() {
        let (dir, files) = make_tree("all");
        assert_eq!(walk(&dir), files);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn walk_of_a_missing_folder_is_empty() {
        let (dir, _) = make_tree("missing");
        assert!(walk(&dir.join("does-not-exist")).is_empty());
        assert!(walk(&dir.join("empty")).is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}
