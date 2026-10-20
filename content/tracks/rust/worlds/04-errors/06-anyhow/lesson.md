In an application, you rarely match on errors. You want one type that holds anything, plus a trail of "what was I doing". That is **anyhow**.

```rust
use anyhow::{bail, Context, Result};

fn load(name: &str) -> Result<String> {
    if name.is_empty() {
        bail!("name must not be empty");           // quick Err
    }
    std::fs::read_to_string(name)
        .with_context(|| format!("reading {name}")) // add context
}
```

- **`anyhow::Result<T>`** is `Result<T, anyhow::Error>`. Any error converts with `?`.
- **`.context("...")`** wraps an error in a higher-level message. The original stays as the **root cause**.
- **`bail!("...")`** returns an error right away. It takes format args like `println!`.
- Apps use anyhow. Libraries use thiserror, so callers can match.

- **`(0..=100).contains(&n)`** asks whether `n` is in a range, ends included. `!` in front means outside. `n < 0 || n > 100` says the same thing.

## Your turn

- Open `src/lib.rs`.
- In `parse_port`, add context to the parse.
- In `check_percent`, `bail!` when the number is outside 0 to 100.
