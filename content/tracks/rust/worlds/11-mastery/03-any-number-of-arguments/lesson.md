A word list in your tests reads `vec!["the".to_string(), "quick".to_string(), "fox".to_string()]`, and the `.to_string()`s are most of the line. You want `strings!["the", "quick", "fox"]`, with as many words as you like. A function cannot do that: Rust functions take a fixed number of arguments. A macro can.

**Say it repeats.** In a pattern, `$( ... ),*` means "this part, any number of times, with commas between". So `$( $s:expr ),*` matches zero, one or fifty expressions, and every one of them is captured as `$s`. This is called **repetition**.

**Star or plus.** The mark after the repetition says how many: `*` is zero or more, `+` is one or more. `vec![]` is fine with zero items, so `strings!` uses `*`.

**Repeat in the template too.** In the template, `$( ... )` with `$s` inside is copied once per captured `$s`, and the mark after it (`,*`, `;*` or plain `*`) says what goes between the copies. Whatever you write around `$s` is copied with it.

```rust
#[macro_export]
macro_rules! total {
    ($($x:expr),*) => {
        0 $( + $x )*
    };
}

assert_eq!(total!(4, 2, 3), 9);   // 0 + 4 + 2 + 3
assert_eq!(total!(), 0);          // zero copies: just 0
```

**Read the copy.** `total!(4, 2, 3)` became `0 + 4 + 2 + 3`: the part inside `$( )` was `+ $x`, copied three times with nothing between. For a `Vec` you want the copies inside `vec![ ]` with commas between them.

## Your turn

- In `strings!`, replace the stub with a `vec![ ]` that holds one `$s.to_string()` per argument, separated by commas.
