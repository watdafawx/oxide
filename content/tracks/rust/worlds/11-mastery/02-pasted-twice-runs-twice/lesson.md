`square!` works on numbers, so you try it on a call: `square!(next())`, where `next()` hands out ticket numbers 3, 4, 5 and so on. You expect 9. The test gets 12 and says `next()` ran twice.

**What was pasted.** The template `$x * $x` pastes the code you wrote, not its value. `square!(next())` becomes `next() * next()`, two calls, 3 times 4. A function would have run `next()` once and received the 3.

**The word for it.** A **side effect** is anything code does besides handing back a value: printing, adding to a counter, taking the next ticket. A plain number like `4` has none, so pasting it twice is harmless; `next()` has one, so pasting it twice does the side effect twice.

**Run it once, keep the answer.** Inside the template, store the argument first: `let v = $x;` runs the code once, then the template uses `v` as often as it likes. A `let` is a statement, so the template must become a block, `{ ... }`, whose last line is its value.

```rust
#[macro_export]
macro_rules! at_most_15 {
    ($x:expr) => {{
        let v = $x;                 // the argument runs here, once
        if v > 15 { 15 } else { v }
    }};
}
```

**Why two pairs of braces.** The outer `{ }` belongs to the arm, like the parentheses around the pattern; the compiler throws it away. The inner `{ }` is the block that holds the `let` and gives back its last line, so the whole call is still one expression you can put after `=`.

**Your v stays yours.** A `v` inside the template never clashes with a `v` in the caller's code. Rust keeps the names a macro makes apart from the caller's names, which is called **hygiene**.

## Your turn

- Rewrite the `square!` template as a block that runs `$x` once, stores it, and multiplies the stored value by itself.
