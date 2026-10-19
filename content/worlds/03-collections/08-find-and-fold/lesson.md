Often you don't want a new list, you want **an answer**: the first match, whether anything is negative, the longest word. Iterators have a consumer for each.

```rust
let nums = [4, 8, 15, 16, 23, 42];

nums.iter().find(|n| **n > 10);          // Some(&15): first match
nums.iter().position(|n| *n == 16);      // Some(3): index of first match
nums.iter().any(|n| *n > 40);            // true
nums.iter().all(|n| *n > 0);             // true

let total = nums.iter().fold(0, |acc, n| acc + n);   // 108
let words = ["pear", "banana", "fig"];
let longest = words.iter().max_by_key(|w| w.len());  // Some(&"banana")
```

- **`find`**: first item matching, as `Option<&T>`. **`position`**: same, but the index.
- **`any` / `all`**: stop as soon as the answer is known.
- **`fold(start, |acc, item| ...)`**: carry a value through the list. The closure returns the next `acc`. Any loop that updates one variable can be a fold. (For plain sums, use `sum()`.)
- **`max_by_key(|x| key)`** / **`min_by_key`**: item with the biggest/smallest key. Empty list gives `None`.

## Your turn

- Open `src/lib.rs`.
- Six small functions, each one consumer. Mind the return types: some want `.copied()`.
