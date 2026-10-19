**Why now:** the chunk indexer registers dozens of block types and chunk sizes, and writing each one by hand is the boilerplate a macro removes.

Macros write code before the compiler checks it. `vec![]` and `println!` are macros, and **`macro_rules!`** lets you make your own.

A macro is a list of arms: **pattern => expansion**. The compiler matches your call against the pattern, then pastes the expansion in place.

- `$x:expr` captures one expression. `$k:expr => $v:expr` also matches the literal `=>` between them.
- The expansion can use `$x` as many times as it likes. If `$x` has side effects, bind it once with `let`.
- `#[macro_export]` makes a macro usable from outside its module.
- Wrap multi-statement expansions in an extra `{ }` so the macro is one expression.

```rust
#[macro_export]
macro_rules! chunk_area {
    ($side:expr) => {{
        let s = $side;
        s * s
    }};
}

assert_eq!(chunk_area!(8 + 8), 256);
```

## Your turn

In `src/lib.rs`:

- `square!`: replace the stub expansion so it returns `x * x`, evaluating `x` only once.
- `hashmap!`: build a `HashMap` and insert the one `$k => $v` pair.
