A broken todo file gives the error `invalid digit found in string`. True, but which line, and which field? The low-level error is right and useless on its own. anyhow lets each layer of your code add a sentence about what it was doing.

**context wraps it.** `.context("...")` on a `Result` puts your message on top of the error. The original stays underneath as the **root cause**, so nothing is lost. The method comes from anyhow's `Context` trait, a promise anyhow keeps for every `Result`, which is how a new method can appear on a type you did not write.

**Reading the chain.** `err.to_string()` shows only the top message. `format!("{err:#}")`, with a `#`, shows the whole chain joined by `: `, top first. `err.root_cause()` hands you the error at the bottom.

**with_context waits.** `.with_context(|| format!("line {n}"))` takes a closure, the nameless function from the collections world, and only runs it when there is an error. With `.context(format!(...))` the `format!` would build a new `String` for every good line too. Fixed text goes in `context`; text you build goes in `with_context`.

```rust
use anyhow::{Context, Result};

fn parse_repeat(text: &str) -> Result<u32> {
    let n: u32 = text.trim().parse().context("repeat must be a number")?;
    Ok(n)
}
```

**For the curious.** `Context` works on `Option` too: `.context("no such task")` turns a `None` into an error with that message, which saves a `match`.

## Your turn

- In `parse_priority`, add `.context("priority must be a number")` to the parse.
- In `priority_on_line`, call `parse_priority` and add `.with_context(|| ...)` with the message `line <n>`.
