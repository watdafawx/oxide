**Why now:** the chunk indexer registers dozens of block types and chunk sizes, and writing each one by hand is the boilerplate a macro removes.

Macros write code before the compiler checks it. `vec![]` and `println!` are macros, and **`macro_rules!`** lets you make your own.

A macro is a list of arms: **pattern => expansion**. The compiler matches your call against the pattern, then pastes the expansion in place.

- `$x:expr` captures one expression.
- The expansion can use `$x` as many times as it likes. If `$x` has side effects, bind it once with `let`.
- `#[macro_export]` makes a macro usable from outside its module.
- Wrap multi-statement expansions in an extra `{ }` so the macro is one expression (see the double braces below).

```rust
#[macro_export]
macro_rules! clamp_chunk {
    ($x:expr) => {{
        let v = $x;                  // evaluated once, here
        if v > 15 { 15 } else { v }
    }};
}

assert_eq!(clamp_chunk!(20), 15);
assert_eq!(clamp_chunk!(3 + 4), 7);
```

Note the **double braces** `{{ ... }}`: the outer pair is the macro's arm, the inner pair is a block that holds the `let` and gives back its last line.

## Your turn

In `src/lib.rs`:

- `square!`: replace the stub expansion so it returns `x * x`, evaluating `x` only once.
