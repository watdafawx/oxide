The very first time someone opens the todo list there is no `todo.txt` yet. That is not a broken file, it is an empty list, and the app should start with no tasks instead of an error. A file you may not read, though, is still a real problem.

**Ask what kind.** `e.kind()` on an `io::Error` returns an **`io::ErrorKind`**, a standard enum that names the kind of failure: `NotFound`, `PermissionDenied`, and a few dozen more. You compare it with `==`, like any value.

**A match guard.** An arm can carry an `if` after its pattern: `Err(e) if e.kind() == io::ErrorKind::NotFound => ...`. That arm only matches when the pattern fits *and* the condition is true. If the condition is false, Rust moves on and tries the next arm.

**Pass the rest on.** A last arm, `Err(e) => ...`, catches every other kind of failure. Those are real problems, so they still go back to the caller, with the context that names the file.

```rust
fn delete_backup(path: &Path) -> Result<()> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(()), // already gone: fine
        Err(e) => Err(e).with_context(|| format!("could not delete {}", path.display())),
    }
}
```

**Why so picky.** Treating every error as "empty" would be easier to write, and wrong: if the file exists but cannot be read, an empty list would be saved over it on exit, and every task would be gone.

## Your turn

- In `load_or_empty`, `match` on `fs::read_to_string(path)`.
- `Ok(text)` gives `Ok(text)`. A `NotFound` error gives `Ok(String::new())`.
- Any other error returns `Err(e)` with the context `could not read <path>`.
