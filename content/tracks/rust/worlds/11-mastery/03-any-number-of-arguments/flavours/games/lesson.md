The chunk indexer's block list reads `vec!["stone".to_string(), "dirt".to_string(), "gravel".to_string()]`, and the `.to_string()`s are most of the line. You want `block_names!["stone", "dirt", "gravel"]`, with as many blocks as you like. A function cannot do that: Rust functions take a fixed number of arguments. A macro can.

**Say it repeats.** In a pattern, `$( ... ),*` means "this part, any number of times, with commas between". So `$( $s:expr ),*` matches zero, one or fifty expressions, and every one of them is captured as `$s`. This is called **repetition**.

**Star or plus.** The mark after the repetition says how many: `*` is zero or more, `+` is one or more. `vec![]` is fine with zero items, so `block_names!` uses `*`.

**Repeat in the template too.** In the template, `$( ... )` with `$s` inside is copied once per captured `$s`, and the mark after it (`,*`, `;*` or plain `*`) says what goes between the copies. Whatever you write around `$s` is copied with it.

```rust
#[macro_export]
macro_rules! total_items {
    ($($x:expr),*) => {
        0 $( + $x )*
    };
}

assert_eq!(total_items!(64, 32, 5), 101);   // 0 + 64 + 32 + 5
assert_eq!(total_items!(), 0);              // zero copies: just 0
```

**Read the copy.** `total_items!(64, 32, 5)` became `0 + 64 + 32 + 5`: the part inside `$( )` was `+ $x`, copied three times with nothing between. For a `Vec` you want the copies inside `vec![ ]` with commas between them.

## Your turn

- In `block_names!`, replace the stub with a `vec![ ]` that holds one `$s.to_string()` per argument, separated by commas.
