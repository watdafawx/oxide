**Why now:** the Inventory is the heart of your plugin, and an inventory is a list that grows: pick up an item, it goes on the end. In Rust that list is a **`Vec<T>`**.

```rust
let mut hotbar = vec!["sword", "pickaxe"];  // vec! makes one with items
hotbar.push("torch");                       // pick something up
println!("{}", hotbar.len());               // 3
println!("{}", hotbar[0]);                  // "sword" (panics if out of range!)
println!("{:?}", hotbar.get(9));            // None: the safe version
for item in &hotbar {                       // borrow, don't consume
    println!("holding {item}");
}
```

- A `Vec` lives on the heap and grows. It must be `mut` to push.
- **`v[i]`** panics when `i` is too big, like reaching for slot 40 in a 36-slot inventory. **`v.get(i)`** gives an `Option`, so you decide what happens.
- **`for x in &v`** borrows each item. `for x in v` would *move* the whole Vec.
- Functions that only read take a slice: `&[u32]`. A `&Vec<u32>` coerces into it.

## Your turn

- Open `src/lib.rs`.
- `starter_kit`: push the two missing items onto the list.
- `third_slot`: use `get`. `total_items`: loop with `for`.
