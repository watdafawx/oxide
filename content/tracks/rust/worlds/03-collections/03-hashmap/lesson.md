A `Vec` finds things by position. A **`HashMap<K, V>`** finds them by key: price by item name, score by player, count by word.

```rust
use std::collections::HashMap;

let mut ages = HashMap::new();
ages.insert("Ada".to_string(), 36);
ages.insert("Linus".to_string(), 28);

println!("{:?}", ages.get("Ada"));    // Some(36)
println!("{:?}", ages.get("Bob"));    // None

let mut counts: HashMap<String, u32> = HashMap::new();
for w in ["a", "b", "a"] {
    *counts.entry(w.to_string()).or_insert(0) += 1;
}
// {"a": 2, "b": 1}   (order is not guaranteed)
```

- **`insert`** adds or replaces. **`get`** returns `Option<&V>`.
- **`entry(key).or_insert(default)`** means "give me the slot for this key, creating it if missing". It returns `&mut V`, so `*... += 1` counts.
- **Iteration order is random.** Never rely on it; sort, or use a `BTreeMap` (later quest).

## Your turn

- Open `src/lib.rs`.
- `price_of`: use `get`. `stock_map`: insert every pair.
- `count_words`: split on whitespace and count with `entry`.
