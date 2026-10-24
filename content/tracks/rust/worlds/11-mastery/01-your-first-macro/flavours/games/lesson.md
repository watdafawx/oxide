The chunk indexer works out areas all over the place: a chunk is 16 by 16 blocks, a region is 32 by 32 chunks, a slab is half a block. `side * side` is short, but `(16 + border) * (16 + border)` is not, and you would rather write `chunk_area!(16 + border)`. A name with `!` is a macro, like `vec!` and `format!`, and `macro_rules!` lets you write your own.

**Code that writes code.** A **macro** runs before the compiler checks your program: it replaces the call with code. `vec![1, 2]` turns into code that builds a `Vec` and pushes two items, and only then does the compiler look at types.

**Pattern, then template.** `macro_rules!` lists **arms**, each written `(pattern) => { template };`. The compiler matches your call against the pattern and pastes the template in its place. `$x:expr` in a pattern captures one **expression**, any piece of code that has a value (`64`, `8 + 8`, `next_id()`), and `$x` in the template pastes it back.

```rust
#[macro_export]
macro_rules! two_stacks {
    ($x:expr) => {
        $x + $x
    };
}

assert_eq!(two_stacks!(64), 128);
assert_eq!(two_stacks!(0.25), 0.5);
```

**Why not a function?** `fn chunk_area(side: i32) -> i32` takes `i32` only; slabs would need a second function, or a generic one with the `Mul` trait from world 05. A macro never sees a type: it copies code into the template, and the compiler checks the result, so `chunk_area!(16)` and `chunk_area!(0.5)` both work. That is a small win here; the next quests show things only a macro can do, such as taking any number of block names.

**One expression stays one.** `$x` pastes a whole expression, not loose text. `two_stacks!(8 + 8)` is `(8 + 8) + (8 + 8)`, and with `*` instead of `+` you get 256, not the 80 that `8 + 8 * 8 + 8` would give.

**Share it.** `#[macro_export]` above the macro is an attribute, a note for the compiler (as in world 02's `#[derive]`). It makes the macro usable everywhere in this crate and from other crates. The starter already has it.

## Your turn

- In `chunk_area!`, change the template so it multiplies the captured expression by itself.
