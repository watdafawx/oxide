**Why now:** the chunk indexer computes areas, heights and block counts in a dozen places, and each is the same few lines. A **macro** writes that code for you at compile time. Functions work on values, macros work on **code**, so they can do things functions cannot (take any expression, repeat a pattern, build items).

```rust
macro_rules! top_block {
    ($y:expr) => {{
        let h = $y;                  // evaluate the argument ONCE
        if h > 255 { 255 } else { h }
    }};
}

let a = top_block!(300);             // 255
let b = top_block!(64 + 64);         // 128: `64 + 64` is one expression
```

- `macro_rules!` matches the code you pass against **patterns** and expands to a template.
- `$x:expr` captures one expression. `$x` pastes it back in.
- A macro that pastes `$x` twice runs it twice. `top_block!(next_id())` with `h` pasted twice would call `next_id` two times. Bind to a `let` first.
- The double braces `{{ ... }}` make the expansion a block, so `let v` stays private.
- `#[macro_export]` makes the macro usable from other crates (and from the tests).

## Your turn

In `src/lib.rs`: finish the `chunk_area!` macro so both tests pass.
