`chunk_area!` works on numbers, so you try it on a call: `chunk_area!(next_side())`, where `next_side()` reads the next chunk size from the world file: 3, then 4, then 5. You expect 9. The test gets 12 and says `next_side()` ran twice.

**What was pasted.** The template `$x * $x` pastes the code you wrote, not its value. `chunk_area!(next_side())` becomes `next_side() * next_side()`, two reads, 3 times 4. A function would have run `next_side()` once and received the 3.

**The word for it.** A **side effect** is anything code does besides handing back a value: printing, adding to a counter, reading the next entry of a file. A plain number like `16` has none, so pasting it twice is harmless; `next_side()` has one, so pasting it twice does the side effect twice.

**Run it once, keep the answer.** Inside the template, store the argument first: `let v = $x;` runs the code once, then the template uses `v` as often as it likes. A `let` is a statement, so the template must become a block, `{ ... }`, whose last line is its value.

```rust
#[macro_export]
macro_rules! cap_at_64 {
    ($x:expr) => {{
        let v = $x;                 // the argument runs here, once
        if v > 64 { 64 } else { v }
    }};
}
```

**Why two pairs of braces.** The outer `{ }` belongs to the arm, like the parentheses around the pattern; the compiler throws it away. The inner `{ }` is the block that holds the `let` and gives back its last line, so the whole call is still one expression you can put after `=`.

**Your v stays yours.** A `v` inside the template never clashes with a `v` in the caller's code. Rust keeps the names a macro makes apart from the caller's names, which is called **hygiene**.

## Your turn

- Rewrite the `chunk_area!` template as a block that runs `$x` once, stores it, and multiplies the stored value by itself.
