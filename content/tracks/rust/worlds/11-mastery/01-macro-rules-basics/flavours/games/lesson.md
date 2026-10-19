**Why now:** the chunk indexer computes areas, heights and block counts in a dozen places, and each is the same few lines. A **macro** writes that code for you at compile time. Functions work on values, macros work on **code**, so they can do things functions cannot (take any expression, repeat a pattern, build items).

```rust
macro_rules! chunk_area {
    ($side:expr) => {{
        let v = $side;      // evaluate the argument ONCE
        v * v
    }};
}

let area = chunk_area!(16);          // 256
let also = chunk_area!(15 + 1);      // 256: `15 + 1` is one expression
```

- `macro_rules!` matches the code you pass against **patterns** and expands to a template.
- `$x:expr` captures one expression. `$x` pastes it back in.
- A macro that pastes `$x` twice runs it twice. `chunk_area!(next_id())` would call `next_id` two times. Bind to a `let` first.
- The double braces `{{ ... }}` make the expansion a block, so `let v` stays private.
- `#[macro_export]` makes the macro usable from other crates (and from the tests).

## Your turn

In `src/lib.rs`: finish the `chunk_area!` macro so both tests pass.
