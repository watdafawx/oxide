A game needs numbers that change (score) and values you set once (max HP). Rust makes you say which is which.

```rust
let name = "ferris";       // type inferred: &str. Never changes.
let mut lives = 3;         // `mut` = allowed to change
lives -= 1;                // now 2
let lives = lives * 10;    // shadowing: a brand-new `lives` (20)
```

- **`let`** makes a variable. It is **immutable** unless you add `mut`.
- Rust **infers** the type (`3` is an `i32`), so you rarely write it.
- **Shadowing** reuses a name for a new variable, even with a different type. The old one is gone.

## Your turn

- Open `src/lib.rs`.
- In `score_with_bonus`, make `score` mutable and add 5.
- In `trimmed_len`, shadow `text` with `text.trim()` and return its `.len()`.
