**Why now:** your crafting calculator starts with an item stack. The count changes when you pick things up, the item's name never does, and a typo like `"  oak_log "` needs cleaning before you can compare it. Rust makes you say which values may change.

```rust
let item = "oak_log";       // type inferred: &str. Never changes.
let mut count = 3;          // `mut` = allowed to change
count += 1;                 // now 4
let count = count * 16;     // shadowing: a brand-new `count` (64)
```

- **`let`** makes a variable. It is **immutable** unless you add `mut`.
- Rust **infers** the type (`3` is an `i32`), so you rarely write it.
- **Shadowing** reuses a name for a new variable, even with a different type. The old one is gone.

## Your turn

- Open `src/lib.rs`.
- In `stack_after_pickup`, make `stack` mutable and add 5.
- In `item_name_len`, shadow `name` with `name.trim()` and return its `.len()`.
