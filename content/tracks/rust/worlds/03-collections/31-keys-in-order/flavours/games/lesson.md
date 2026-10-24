The `/inv` printout lists every item and its count. With a `HashMap` the lines come out in a different order on different runs, and players think items vanished. The fix is a map that keeps its keys in order.

**A sorted map.** A **`BTreeMap`** has the same methods as a `HashMap`, `insert`, `get` and `entry`, but keeps its keys sorted: text a to z, numbers small to big. Walk it and the keys always come out in that order. It also needs a `use`: `use std::collections::BTreeMap;`.

**Walk the pairs.** `map.iter()` is an iterator over `(key, value)` pairs, both as references. A closure takes the pair apart with a pattern, `|(item, n)|`, as with `enumerate`.

```rust
use std::collections::BTreeMap;

let mut kills = BTreeMap::new();
kills.insert("zombie".to_string(), 6);
kills.insert("creeper".to_string(), 2);
let lines: Vec<String> = kills.iter().map(|(mob, k)| format!("{mob}: {k}")).collect();
// ["creeper: 2", "zombie: 6"]: sorted by key
```

**Which one?** A `HashMap` is a little faster for lookups. A `BTreeMap` is the one to pick when the result is printed, compared in a test, or saved to a file, because the same data always gives the same output.

**Same counter, new map.** `sorted_inventory` is the counter from "Count each drop" with one change: the map is a `BTreeMap::new()`. `entry` and `or_insert` work exactly as before.

## Your turn

- In `sorted_inventory`, count each item with `entry` into the `BTreeMap`.
- In `inventory_lines`, walk the map and format each pair as `"item xcount"`, like `"torch x3"`.
