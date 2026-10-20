**Why now:** the top of your launcher doesn't match on `ManifestError` variants; it just wants to tell the player "couldn't load the pack: why". One catch-all error type plus a trail of "what was I doing" is exactly what **anyhow** gives you.

```rust
use anyhow::{bail, Context, Result};

fn load(name: &str) -> Result<String> {
    if name.is_empty() {
        bail!("pack name must not be empty");        // quick Err
    }
    std::fs::read_to_string(name)
        .with_context(|| format!("reading {name}"))   // add context
}
```

- **`anyhow::Result<T>`** is `Result<T, anyhow::Error>`. Any error converts with `?`.
- **`.context("...")`** wraps an error in a higher-level message. The original stays as the **root cause**.
- **`bail!("...")`** returns an error right away. It takes format args like `println!`.
- Apps use anyhow. Libraries use thiserror, so callers can match.

- **`(0..=100).contains(&n)`** asks whether `n` is in a range, ends included. `!` in front means outside. `n < 0 || n > 100` says the same thing.

## Your turn

- Open `src/lib.rs`.
- In `parse_version`, add context to the parse.
- In `check_ram_percent`, `bail!` when the number is outside 0 to 100.
