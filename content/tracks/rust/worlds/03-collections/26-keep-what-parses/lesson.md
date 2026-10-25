A staff member typed the morning counts as `12, 6, x, 1`: one entry is a typo. The tool should keep the numbers it can read and skip the rest. Parsing each piece gives a `Result`, and you want a list of only the good ones.

**From Result to Option.** `parse` answers `Ok(12)` or `Err(...)`, as in the Foundations world. `.ok()` turns that into an `Option`: `Ok(12)` becomes `Some(12)`, and an `Err` becomes `None`, dropping the error message. Here you do not care why a piece is bad, only that it is.

**Map and filter in one.** `.filter_map(closure)` is an adaptor: the closure gives an `Option` for each item, `Some` values go on down the chain unwrapped, and `None`s are dropped. With plain `map` you would collect a `Vec<Option<u32>>`, which is not what you wanted.

```rust
let ages: Vec<u32> = ["30", "old", "41"].iter().filter_map(|s| s.parse().ok()).collect();
// [30, 41]
```

**Who says u32?** `parse` needs to know what to build. Here the `Vec<u32>` at the end of the chain tells it, so no turbofish is needed. When nothing at the end decides, write it on the call: `parse::<u32>()`.

**Counting the bad ones.** `.is_err()` on a `Result` is `true` for an `Err`. A `filter` on that, then `count`, says how many pieces failed.

## Your turn

- In `parse_counts`, split on commas and `filter_map` each trimmed piece through `parse().ok()`.
- In `count_bad`, keep the pieces whose `parse::<u32>()` is an error, and count them.
