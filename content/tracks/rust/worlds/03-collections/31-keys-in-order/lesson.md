The end-of-day sales report lists every product and how many sold. With a `HashMap` the lines come out in a different order on different runs, and the owner keeps thinking products went missing. The fix is a map that keeps its keys in order.

**A sorted map.** A **`BTreeMap`** has the same methods as a `HashMap`, `insert`, `get` and `entry`, but keeps its keys sorted: text a to z, numbers small to big. Walk it and the keys always come out in that order. It also needs a `use`: `use std::collections::BTreeMap;`.

**Walk the pairs.** `map.iter()` is an iterator over `(key, value)` pairs, both as references. A closure takes the pair apart with a pattern, `|(name, n)|`, as with `enumerate`.

```rust
use std::collections::BTreeMap;

let mut hours = BTreeMap::new();
hours.insert("bo".to_string(), 6);
hours.insert("ann".to_string(), 8);
let lines: Vec<String> = hours.iter().map(|(who, h)| format!("{who} {h}h")).collect();
// ["ann 8h", "bo 6h"]: sorted by key
```

**Which one?** A `HashMap` is a little faster for lookups. A `BTreeMap` is the one to pick when the result is printed, compared in a test, or saved to a file, because the same data always gives the same output.

**Same counter, new map.** `sales_report` is the counter from "Count each one" with one change: the map is a `BTreeMap::new()`. `entry` and `or_insert` work exactly as before.

## Your turn

- In `sales_report`, count each sale with `entry` into the `BTreeMap`.
- In `report_lines`, walk the map and format each pair as `"name: count"`.
