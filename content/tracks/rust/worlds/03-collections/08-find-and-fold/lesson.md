Often you don't want a new list, you want **an answer**: the first match, whether anything is negative. Iterators have a consumer for each.

```rust
let nums = [4, 8, 15, 16, 23, 42];

nums.iter().find(|n| **n > 10);          // Some(&15): first match
nums.iter().position(|n| *n == 16);      // Some(3): index of first match
nums.iter().any(|n| *n > 40);            // true
nums.iter().all(|n| *n > 0);             // true
```

- **`find`**: first item matching, as `Option<&T>`. **`position`**: same, but the index.
- **`any` / `all`**: stop as soon as the answer is known. `all` on an empty list is `true`.
- The closure gets a reference to each item, so you often write `*n` or use `.copied()`.

## Your turn

- Open `src/lib.rs`.
- Four small functions, each one consumer. Mind the return types: some want `.copied()`.
