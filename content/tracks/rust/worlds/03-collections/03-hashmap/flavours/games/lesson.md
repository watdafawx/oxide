**Why now:** a chest holds hundreds of items and your plugin must answer "how many diamonds are in there?" instantly. Scanning a `Vec` by position is slow and awkward. A **`HashMap<K, V>`** finds things by key: price by item name, count by item, stack by chest.

```rust
use std::collections::HashMap;

let mut chest = HashMap::new();
chest.insert("diamond".to_string(), 3);
chest.insert("coal".to_string(), 40);

println!("{:?}", chest.get("diamond"));   // Some(3)
println!("{:?}", chest.get("emerald"));   // None

let mut kills: HashMap<String, u32> = HashMap::new();
for mob in ["zombie", "creeper", "zombie"] {
    *kills.entry(mob.to_string()).or_insert(0) += 1;
}
// {"zombie": 2, "creeper": 1}   (order is not guaranteed)
```

- **`insert`** adds or replaces. **`get`** returns `Option<&V>`.
- **`entry(key).or_insert(default)`** means "give me the slot for this key, creating it if missing". It returns `&mut V`, so `*... += 1` counts.
- **Iteration order is random.** Never rely on it; sort, or use a `BTreeMap` (later quest).

- **`a.iter().zip(b)`** walks two lists side by side and yields `(x, y)` pairs, stopping at the shorter one. Handy when two slices line up by position.

## Your turn

- Open `src/lib.rs`.
- `trade_price`: use `get`. `chest_index`: insert every name and count pair.
- `count_drops`: split the kill log on whitespace and count with `entry`.
