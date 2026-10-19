**Why now:** armor trims a hit piece by piece, and the chest screen wants to highlight the fullest stack. One answer is a **running result**, the other is **the biggest item**.

```rust
let hits = [4, 8, 15];
let total = hits.iter().fold(0, |acc, n| acc + n);    // 27

let items = ["bow", "diamond_sword", "axe"];
let longest = items.iter().max_by_key(|w| w.len());   // Some(&"diamond_sword")
```

- **`fold(start, |acc, item| ...)`**: carry a value through the list. The closure returns the next `acc`. Armor works like this: each piece trims what is left. (For plain sums, use `sum()`.)
- **`max_by_key(|x| key)`** / **`min_by_key`**: item with the biggest/smallest key. Empty list gives `None`.

## Your turn

- Open `src/lib.rs`. `ItemStack` is given.
- `damage_after_armor`: a `fold` over the reductions. `biggest_stack`: `max_by_key` on the count.
