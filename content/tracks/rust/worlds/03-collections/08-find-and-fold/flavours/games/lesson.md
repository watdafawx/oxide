**Why now:** your plugin keeps asking one-answer questions about an inventory. Which slot is the first full stack? Is any tool about to break? You don't want a new list, you want **an answer**, and iterators have a consumer for each.

```rust
let stacks = [4, 8, 15, 16, 23, 42];

stacks.iter().find(|n| **n > 10);          // Some(&15): first match
stacks.iter().position(|n| *n == 16);      // Some(3): slot of first match
stacks.iter().any(|n| *n > 40);            // true
stacks.iter().all(|n| *n > 0);             // true
```

- **`find`**: first item matching, as `Option<&T>`. **`position`**: same, but the index.
- **`any` / `all`**: stop as soon as the answer is known. `all` on an empty list is `true`.
- The closure gets a reference to each item, so you often write `*n` or use `.copied()`.

## Your turn

- Open `src/lib.rs`.
- Four small functions, each one consumer. Mind the return types: some want `.copied()`.
