**Why now:** your plugin keeps asking one-answer questions about an inventory. Which slot is the first full stack? Is any tool broken? How much damage gets through armor? Who holds the biggest stack? You don't want a new list, you want **an answer**, and iterators have a consumer for each.

```rust
let stacks = [4, 8, 15, 16, 23, 42];

stacks.iter().find(|n| **n > 10);          // Some(&15): first match
stacks.iter().position(|n| *n == 16);      // Some(3): slot of first match
stacks.iter().any(|n| *n > 40);            // true
stacks.iter().all(|n| *n > 0);             // true

let total = stacks.iter().fold(0, |acc, n| acc + n);   // 108
let items = ["bow", "diamond_sword", "axe"];
let longest = items.iter().max_by_key(|w| w.len());    // Some(&"diamond_sword")
```

- **`find`**: first item matching, as `Option<&T>`. **`position`**: same, but the index.
- **`any` / `all`**: stop as soon as the answer is known.
- **`fold(start, |acc, item| ...)`**: carry a value through the list. The closure returns the next `acc`. Armor works like this: each piece trims what is left. (For plain sums, use `sum()`.)
- **`max_by_key(|x| key)`** / **`min_by_key`**: item with the biggest/smallest key. Empty list gives `None`.

## Your turn

- Open `src/lib.rs`. `ItemStack` is given.
- Six small functions, each one consumer. Mind the return types: some want `.copied()`.
