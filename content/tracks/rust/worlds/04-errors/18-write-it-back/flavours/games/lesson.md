The launcher can read its manifest. Now, when the player adds a mod, the launcher has to write the manifest back, or the new mod is gone the next time the game starts.

**The other direction.** `fs::write(path, text)` creates the file, or replaces everything in it, with your text. It returns `Result<(), io::Error>`. The **`()`** is the empty value: on success there is nothing to hand back except "it worked".

**One line each.** The file holds one mod per line, and every line ends with `\n`, the line-break character. Build the whole text in one `String` first, then write it in one go: `text.push_str(line)` adds a line and `text.push('\n')` adds the break.

**Replace means replace.** `fs::write` throws away whatever the file held before. That is why the last quest refused to treat an unreadable file as empty: saving would have wiped it.

**Writing fails too.** The folder may not exist, or the disk may be full. Give the error the same kind of context as reading: which file, and what you were doing.

```rust
fn save_note(path: &Path, note: &str) -> Result<()> {
    fs::write(path, format!("{note}\n"))
        .with_context(|| format!("could not write {}", path.display()))
}
```

## Your turn

- In `save_mods`, start with `let mut text = String::new();` and add each mod followed by `\n`.
- Write `text` to `path` with `fs::write`, and add the context `could not write <path>`.
