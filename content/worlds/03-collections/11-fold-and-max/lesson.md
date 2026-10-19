Some answers aren't "the first match": they are a **running result** (a price after several discounts) or **the biggest item**.

```rust
let nums = [4, 8, 15];
let total = nums.iter().fold(0, |acc, n| acc + n);   // 27

let words = ["pear", "banana", "fig"];
let longest = words.iter().max_by_key(|w| w.len());  // Some(&"banana")
```

- **`fold(start, |acc, item| ...)`**: carry a value through the list. The closure returns the next `acc`. Any loop that updates one variable can be a fold. (For plain sums, use `sum()`.)
- **`max_by_key(|x| key)`** / **`min_by_key`**: item with the biggest/smallest key. Empty list gives `None`.

## Your turn

- Open `src/lib.rs`.
- `after_discounts`: a `fold` over the percents. `longest_word`: `max_by_key` on the length.
