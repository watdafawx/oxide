`match` checks a value against patterns and the compiler makes sure you covered **every** case.

```rust
let msg = match hp {
    0 => "dead",
    1..=20 => "critical",
    n if n % 2 == 0 => "even and healthy",   // guard: extra `if`
    _ => "healthy",                          // catch-all
};

if let Some(x) = maybe_number {              // only care about one pattern
    println!("{x}");
}
```

- **Exhaustive**: miss a case and it won't compile. Add a new enum variant later and the compiler points at every `match` to fix.
- Arms are tried **top to bottom**. First match wins.
- `n if cond` is a **guard**: binds `n`, then checks the condition.
- **`if let`** is `match` with one arm and an implicit "ignore the rest".
- `match` is an expression: it returns a value, so arms need the same type.
- **`A | B =>`** lets two patterns share one arm: `0 | 1 => "tiny"`.
- To hand back text you only borrowed, copy it: **`.clone()`** on a `&String` gives an owned `String`, and **`String::new()`** is an empty one.
- A single `char` uses single quotes: `'A'`. Double quotes make text.

## Your turn

- Open `src/lib.rs`. `Command` is given.
- Fill all four functions. Run tests after each one.
