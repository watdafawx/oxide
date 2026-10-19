**Why now:** an enchanted book can carry the same enchantment twice by accident, two swords share some enchantments, and the inventory printout must come out in the same order every time or players think items vanished. Two more containers solve exactly that, both from `std::collections`:

- **`HashSet<T>`**: a bag with no repeats. Fast "have I seen this?" and set maths.
- **`BTreeMap<K, V>`**: a `HashMap` that keeps its keys **sorted**. Iterate it and the output is always in the same order.

```rust
use std::collections::{BTreeMap, HashSet};

let sword: HashSet<&str> = ["sharpness", "unbreaking", "sharpness"].into_iter().collect();
let axe: HashSet<&str> = ["efficiency", "unbreaking"].into_iter().collect();
// sword = {"sharpness", "unbreaking"}: the repeat vanished

sword.contains("sharpness");                       // true
sword.len();                                       // 2
let both: Vec<_> = sword.intersection(&axe).collect();   // "unbreaking"

let mut bag = BTreeMap::new();
bag.insert("torch", 3);
bag.insert("arrow", 24);
for (item, n) in &bag {                            // arrow first, then torch: sorted
    println!("{item}: {n}");
}
```

- `HashSet`: `insert`, `contains`, `len`, `intersection`, `union`, `difference`. Order is random, so `sort()` a `Vec` copy if order matters.
- `BTreeMap` has the same API as `HashMap` (`insert`, `get`, `entry`). Use it when you print or compare results.

## Your turn

- Open `src/lib.rs`.
- `distinct_enchants`: collect into a `HashSet`. `shared_enchants`: intersect two sets.
- `sorted_inventory`: count items into a `BTreeMap`.
