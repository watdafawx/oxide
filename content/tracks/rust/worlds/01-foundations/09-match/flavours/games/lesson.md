**Why now:** your calculator has to label items by rarity and react to inventory actions (add, remove, rename). `match` checks a value against patterns and the compiler makes sure you covered **every** case, so a new action can never be silently ignored.

```rust
let tier = match enchants {
    0 => "plain",
    1..=2 => "magic",
    n if n % 2 == 0 => "even and strong",   // guard: extra `if`
    _ => "strong",                          // catch-all
};

if let Some(slot) = free_slot {             // only care about one pattern
    println!("{slot}");
}
```

- **Exhaustive**: miss a case and it will not compile. Add a new enum variant later and the compiler points at every `match` to fix.
- Arms are tried **top to bottom**. First match wins.
- `n if cond` is a **guard**: binds `n`, then checks the condition.
- **`if let`** is `match` with one arm and an implicit "ignore the rest".
- `match` is an expression: it returns a value, so arms need the same type.
- **`A | B =>`** lets two patterns share one arm: `0 | 1 => "tiny"`.
- To hand back text you only borrowed, copy it: **`.clone()`** on a `&String` gives an owned `String`, and **`String::new()`** is an empty one.

## Your turn

- Open `src/lib.rs`. `StackAction` is given.
- Fill all four functions. Run tests after each one.
