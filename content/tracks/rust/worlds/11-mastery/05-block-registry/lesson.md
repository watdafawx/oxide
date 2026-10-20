**Why now:** two heavy proc-macro quests are behind you. This one is a breather: the chunk indexer needs a table of blocks (name and id), and a small `macro_rules!` makes each entry one line.

Everything you need is already in quests 1 and 2: one arm for one block, repetition for many.

```rust
macro_rules! point {
    ($x:expr, $y:expr) => { Point { x: $x, y: $y } };
}
```

`=>` in a pattern is just a literal token that must appear in the call, and `$( ... ),*` can wrap a whole macro call: `vec![$( point!($x, $y) ),*]`. A struct from your crate is written `$crate::Point { .. }` in the expansion, so it works wherever the macro is used. Inside another macro, call yours as `$crate::name!(..)` so it works from any module.

## Your turn

In `src/lib.rs`:

- `block!`: expand `block!("stone" => 1)` to a `Block`.
- `registry!`: expand a list of `name => id` pairs to a `Vec<Block>`.
