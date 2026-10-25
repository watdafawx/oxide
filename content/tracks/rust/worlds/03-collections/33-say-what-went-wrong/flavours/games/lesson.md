When a player mistypes `/give`, the chat shows the error message. `invalid digit found in string` does not say which argument or what was typed. The plugin's functions return `Result<_, String>`, with a message a player can act on: `bad slot 'nine'`.

**Try the obvious thing.** In a function that returns `Result<u32, String>`, write `text.parse::<u32>()?`. Rust refuses: `parse` fails with a `ParseIntError`, your function promises a `String`, and `?` cannot turn one into the other on its own.

```text
error[E0277]: `?` couldn't convert the error to `String`
```

**Rewrite the error.** `.map_err(closure)` on a `Result` leaves an `Ok` alone and runs the closure on the error inside an `Err`, putting your answer in its place. The closure `|_| format!("bad count '{text}'")` ignores the original error, `_` as in a `match`, and builds your message.

```rust
let level: u32 = typed.parse().map_err(|_| format!("level must be a number, got '{typed}'"))?;
```

**Then pass it on.** After `map_err`, the error is a `String`, so `?` fits again. That pair, `map_err(...)?`, is the everyday way to read a value and give a helpful error in one line.

**For the curious.** `|e| format!("bad count: {e}")` keeps the original message too. The Errors world shows how to make error types that convert by themselves, so `?` works without `map_err`.

## Your turn

- In `parse_count`, try `?` first and read the error. Then parse the trimmed text and `map_err` it to `bad count '<text>'`.
- In `parse_give`, parse both arguments with `map_err(...)?`, with `bad count` and `bad slot` messages, and return `Ok((count, slot))`.
