The todo list's command line reads the hours a task takes. When the input is bad it just prints the reason and asks again; nobody matches on the error. Writing an enum for that is wasted effort, and the **anyhow** crate, already in `Cargo.toml`, gives you one error type for anything instead.

**One type for anything.** An `anyhow::Error` holds any error that keeps the `Error` promise, much like `Box<dyn Error>`, plus a few extras the next quest uses. **`anyhow::Result<T>`** is short for `Result<T, anyhow::Error>`, and `?` converts every error into it.

**bail! is return Err.** `bail!("...")` builds an anyhow error from a message and returns it from the function right away. It takes `{}` like `format!`, so `bail!("{n} is too many")` works.

**The ! marks a macro.** A name ending in `!` is a **macro**: code that writes more code before the program is compiled. That is how `format!`, `write!` and `bail!` accept any number of arguments, which a plain function cannot.

```rust
use anyhow::{bail, Result};

fn parse_repeat(text: &str) -> Result<u32> {
    let n: u32 = text.trim().parse()?;   // a ParseIntError becomes an anyhow::Error
    if n == 0 {
        bail!("repeat every 0 days makes no sense");
    }
    Ok(n)
}
```

**Which one when.** Code other people call, like your parser, keeps a thiserror enum, so callers can match on it. The top of an app, which only shows the message and moves on, uses anyhow. The boss of this world uses both.

## Your turn

- In `parse_hours`, parse the trimmed text as a `u32` with `?`.
- If it is above 24, `bail!` with the message `<n> hours is more than a day`.
- Otherwise return `Ok` with the number.
