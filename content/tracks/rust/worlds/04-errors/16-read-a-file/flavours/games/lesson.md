So far the launcher forgets the modpack when it stops. To remember, it has to read the manifest from a file on disk, `mods.txt`, one mod per line.

**Files can fail.** A program's values vanish when it stops, while a **file** on disk stays. Reading one can fail for reasons your code cannot control: the file is not there, or you may not open it. So reading a file returns a `Result`, like parsing does.

**Path names a place.** **`Path`**, from `std::path`, is the standard library's type for a file location, such as `mods.txt`. Functions take it as `&Path`, and `Path::new("mods.txt")` makes one from text.

**Read it all at once.** `fs::read_to_string(path)`, from `std::fs`, reads the whole file into a `String`. It returns `Result<String, io::Error>`, where **`io::Error`** is the standard library's error for anything input and output.

**Say which file.** An `io::Error` reads `No such file or directory (os error 2)`, but not which file. Add `with_context` from the last quests. `path.display()` turns a `Path` into something `{}` can print; it needs a method because a file name is not always valid text.

**Line by line.** `text.lines()` hands you one line at a time, without the line break, ready for a `for` loop.

```rust
use std::fs;
use std::path::Path;
use anyhow::{Context, Result};

fn has_line(path: &Path, wanted: &str) -> Result<bool> {
    let text = fs::read_to_string(path)
        .with_context(|| format!("could not open {}", path.display()))?;
    for line in text.lines() {
        if line.trim() == wanted {
            return Ok(true);
        }
    }
    Ok(false)
}
```

## Your turn

- In `load_text`, read the file and add the context `could not read <path>`, using `path.display()`.
- In `count_mods`, call `load_text(path)?` and count the lines that are not empty after `trim()`.
