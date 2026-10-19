**Why now:** a real inventory needs editing: shift the pickaxe to slot 1, sort a messy loot pile, throw out the 1-item scraps. `Vec` has a method for each, and your plugin will call them constantly.

```rust
let mut v = vec!["bone", "arrow", "bone", "dirt"];
v.sort();                        // ["arrow", "bone", "bone", "dirt"]
v.dedup();                       // ["arrow", "bone", "dirt"]  (only removes neighbours)
v.insert(1, "torch");            // ["arrow", "torch", "bone", "dirt"]
let gone = v.remove(0);          // gone = "arrow"
println!("{}", v.contains(&"dirt")); // true: contains wants a reference
v.retain(|name| name.len() > 4); // keep only matches: ["torch"]
```

- **`insert(i, x)`** and **`remove(i)`** shift everything after `i`. `remove` hands the item back.
- **`sort()`** works in place on `mut` vectors. **`dedup()`** needs sorted data to catch all repeats.
- **`contains(&x)`** asks "is it in there?".
- **`retain(|x| keep_it)`** keeps items where the closure says `true`. The `|x| ...` is a closure; a later quest unpacks it.

## Your turn

- Open `src/lib.rs`. `ItemStack` (a name and a count) is given.
- Fill in the four functions. Each is 1-3 lines using a method above.
