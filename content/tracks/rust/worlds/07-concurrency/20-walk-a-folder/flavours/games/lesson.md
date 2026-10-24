Before anything can be hashed, the hasher needs the list of files: every jar in `mods/`, the library jars some packs keep in `mods/libs/`, and the configs next to them. The errors world read one file by its `Path`. Now you list a folder's contents and go down into each folder you find.

**List one folder.** `std::fs::read_dir(dir)` opens a folder. It returns a `Result`, an `Err` if the folder is missing or unreadable, and the `Ok` holds an iterator over the folder's **entries**, one per file or subfolder. Each entry is itself a `Result`, because reading one entry can fail too; `if let Ok(entry) = entry` keeps the good ones.

**An owned path.** `entry.path()` gives the entry's full path as a `PathBuf`. A `PathBuf` is to `&Path` what `String` is to `&str`: the owned version, which you can keep in a `Vec` and return. `&path` lends it as a `&Path`, for functions that only read it.

**File or folder.** `path.is_dir()` is `true` for a folder. A file goes straight into the list. A folder is a smaller version of the same problem, so call `walk` on it, the way the smart-pointers world walked a chain: the function calls itself, and a folder with no subfolders is the case that stops it.

**Sort at the end.** `read_dir` hands out entries in whatever order the disk keeps them, which differs between Windows, macOS and Linux. Sorting the `Vec<PathBuf>` makes the list the same on every player's PC, which the launcher's sync and its tests need.

```rust
use std::fs;
use std::path::{Path, PathBuf};

fn instance_folders(dir: &Path) -> Vec<PathBuf> {
    let mut folders = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    folders.push(path);
                }
            }
        }
    }
    folders
}
```

## Your turn

- In `walk`, read the folder and return an empty `Vec` if that fails.
- For each entry: push a file's path, or call `walk` on a folder and push every path it gives back.
- Sort the list before you return it.
