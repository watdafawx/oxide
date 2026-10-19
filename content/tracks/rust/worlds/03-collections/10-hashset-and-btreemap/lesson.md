Two more containers, both from `std::collections`:

- **`HashSet<T>`**: a bag with no repeats. Fast "have I seen this?" and set maths.
- **`BTreeMap<K, V>`**: a `HashMap` that keeps its keys **sorted**. Iterate it and the output is always in the same order.

```rust
use std::collections::{BTreeMap, HashSet};

let a: HashSet<i32> = [1, 2, 3, 3].into_iter().collect();   // {1, 2, 3}
let b: HashSet<i32> = [2, 3, 4].into_iter().collect();

a.contains(&2);                          // true
a.len();                                 // 3
let both: Vec<_> = a.intersection(&b).collect();   // 2 and 3, any order

let mut m = BTreeMap::new();
m.insert("pear", 3);
m.insert("apple", 5);
for (name, n) in &m {                    // apple first, then pear: sorted
    println!("{name}: {n}");
}
```

- `HashSet`: `insert`, `contains`, `len`, `intersection`, `union`, `difference`. Order is random, so `sort()` a `Vec` copy if order matters.
- `BTreeMap` has the same API as `HashMap` (`insert`, `get`, `entry`). Use it when you print or compare results.

## Your turn

- Open `src/lib.rs`.
- `unique_count`: collect into a `HashSet`. `common`: intersect two sets.
- `letter_counts`: count letters into a `BTreeMap`.
