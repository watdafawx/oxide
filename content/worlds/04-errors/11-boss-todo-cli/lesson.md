Time to put it all together: a todo list that survives restarts. Typed errors where callers need to match, **anyhow** where they just report.

```rust
// inside: precise, matchable
#[derive(Debug, thiserror::Error)]
enum TodoError { #[error("no todo with id {0}")] NotFound(u32) }

// outside: one catch-all type, with context
fn run() -> anyhow::Result<()> {
    store.toggle(9)?;                 // TodoError -> anyhow::Error
    Ok(())
}
```

- `toggle` and `parse_command` return **`TodoError`** (thiserror). Tests match on them.
- `load`, `save` and `run` return **`anyhow::Result`**. Add `.with_context()` to I/O errors.
- A **missing file is not an error**: check `io::ErrorKind::NotFound` and return an empty store.
- Tests use `std::env::temp_dir()`, so nothing touches your project.

## Your turn

- Open `src/lib.rs`. `Todo`, `TodoError`, `Command` and the line helpers are given.
- Fill every `todo!()`. Work down the test list: the first one is one line.
- Order that works: `new`, `add`, `toggle`, `parse_command`, `save`, `load`, `run`.
