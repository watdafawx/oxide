When a stock line is bad, the owner sees the error message. `invalid digit found in string` does not say which field or what was typed. The tool's functions return `Result<_, String>`, with a message a person can act on: `bad price '1,50'`.

**Try the obvious thing.** In a function that returns `Result<u32, String>`, write `text.parse::<u32>()?`. Rust refuses: `parse` fails with a `ParseIntError`, your function promises a `String`, and `?` cannot turn one into the other on its own.

```text
error[E0277]: `?` couldn't convert the error to `String`
```

**Rewrite the error.** `.map_err(closure)` on a `Result` leaves an `Ok` alone and runs the closure on the error inside an `Err`, putting your answer in its place. The closure `|_| format!("bad qty '{text}'")` ignores the original error, `_` as in a `match`, and builds your message.

```rust
let age: u32 = typed.parse().map_err(|_| format!("age must be a number, got '{typed}'"))?;
```

**Then pass it on.** After `map_err`, the error is a `String`, so `?` fits again. That pair, `map_err(...)?`, is the everyday way to read a value and give a helpful error in one line.

**For the curious.** `|e| format!("bad qty: {e}")` keeps the original message too. The Errors world shows how to make error types that convert by themselves, so `?` works without `map_err`.

## Your turn

- In `parse_qty`, try `?` first and read the error. Then parse the trimmed text and `map_err` it to `bad qty '<text>'`.
- In `parse_pair`, parse both fields with `map_err(...)?`, with `bad qty` and `bad price` messages, and return `Ok((qty, price))`.
