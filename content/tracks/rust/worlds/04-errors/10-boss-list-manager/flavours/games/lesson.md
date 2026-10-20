**Why now:** this is the launcher's mod list screen: add a mod, switch it on or off, list what you have, and have it all still there tomorrow. Typed errors where callers need to match, **anyhow** where they just report.

```rust
// inside: precise, matchable
#[derive(Debug, thiserror::Error)]
enum ModError { #[error("no mod with id {0}")] NotFound(u32) }

// outside: one catch-all type, with context
fn run() -> anyhow::Result<()> {
    list.toggle(9)?;                  // ModError -> anyhow::Error
    Ok(())
}
```

- `toggle` and `parse_command` return **`ModError`** (thiserror). Tests match on them.
- `load`, `save` and `run` return **`anyhow::Result`**. Add `.with_context()` to I/O errors.
- A **missing file is not an error**: a first launch has no mod list yet. Check `io::ErrorKind::NotFound` and return an empty list.
- Tests use `std::env::temp_dir()`, so nothing touches your project.
- Commands are `add <mod>`, `toggle <id>` (enable if disabled, disable if enabled) and `list`.

- Tools the boss uses: a **match guard** adds a condition to an arm (`Err(e) if e.kind() == ErrorKind::NotFound => ...`). A variant with data is also a function, so `rest.parse().map(Command::Toggle)` wraps an `Ok(n)` into `Ok(Command::Toggle(n))`. `let Some(x) = opt else { return ... };` unwraps or leaves early. `ok_or_else(|| err)` turns `None` into an `Err`.
- More tools: `#[derive(Default)]` is already on the list type, so `Self::default()` builds an empty one. `iter().map(|x| x.id).max()` gives the biggest id as an `Option` (`None` when empty); `unwrap_or(0)` supplies a fallback. `iter_mut()` is `iter()` with `&mut` items, so `iter_mut().find(..)` lets you change what it finds. `std::fs::write(path, text)` writes a `String` to a file. `path.display()` prints a `Path` inside `format!`.
## Your turn

- Open `src/lib.rs`. `ModEntry`, `ModError`, `Command` and the line helpers are given.
- Fill every `todo!()`. Work down the test list: the first one is one line.
- Order that works: `new`, `add`, `toggle`, `parse_command`, `save`, `load`, `run`.

Press Run to play it.
