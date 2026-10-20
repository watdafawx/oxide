**Why now:** the chunk indexer registers dozens of blocks. Typing `"stone".to_string(), "dirt".to_string(), ...` for each is noise. A macro can take **any number** of arguments and apply a pattern to each: that is a **repetition**.

```rust
macro_rules! shout {
    ($($name:expr),* $(,)?) => {
        vec![$($name.to_uppercase()),*]
    };
}

let loud = shout!["stone", "dirt", "gravel",];   // vec!["STONE", "DIRT", "GRAVEL"]

// recursion: peel off the first argument, call yourself on the rest
macro_rules! total {
    ($x:expr) => { $x };                                      // base case first
    ($x:expr, $($rest:expr),+) => { $x + total!($($rest),+) };
}

// literal tokens in a pattern must match exactly: `=>` here
macro_rules! pair {
    ($k:expr => $v:expr) => { ($k, $v) };
}
```

Reading the pattern:

- `$( ... ),*` means "this, repeated, separated by commas, zero or more times".
- `$name:expr` is one captured expression. Inside the expansion, `$( ... ),*` repeats the template once per capture.
- `$(,)?` allows one optional trailing comma.
- `+` instead of `*` means one or more.
- A macro can call itself: `tallest!(a, b, c)` is `max(a, tallest!(b, c))`. Use `$crate::name!` so the call works from other crates.
- Use full paths (`::std::collections::HashMap`) inside macros: the caller may not have imported them.

## Your turn

In `src/lib.rs`, write the three macros:

- `block_names!`: a `Vec<String>` from any number of expressions.
- `block_ids!`: a `HashMap` with one `key => value` pair.
- `tallest!`: the biggest of one or more expressions.
