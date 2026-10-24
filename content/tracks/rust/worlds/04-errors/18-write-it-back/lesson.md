The todo list can read its file. Now, when the user adds a task, the list has to write the file back, or the new task is gone tomorrow.

**The other direction.** `fs::write(path, text)` creates the file, or replaces everything in it, with your text. It returns `Result<(), io::Error>`. The **`()`** is the empty value: on success there is nothing to hand back except "it worked".

**One line each.** The file holds one task per line, and every line ends with `\n`, the line-break character. Build the whole text in one `String` first, then write it in one go: `text.push_str(line)` adds a line and `text.push('\n')` adds the break.

**Replace means replace.** `fs::write` throws away whatever the file held before. That is why the last quest refused to treat an unreadable file as empty: saving would have wiped it.

**Writing fails too.** The folder may not exist, or the disk may be full. Give the error the same kind of context as reading: which file, and what you were doing.

```rust
fn save_note(path: &Path, note: &str) -> Result<()> {
    fs::write(path, format!("{note}\n"))
        .with_context(|| format!("could not write {}", path.display()))
}
```

## Your turn

- In `save_tasks`, start with `let mut text = String::new();` and add each task followed by `\n`.
- Write `text` to `path` with `fs::write`, and add the context `could not write <path>`.
