Almost every app keeps lists: files in a folder, mods in a pack, messages in a chat. In Rust that list is a **`Vec<T>`**.

```rust
let mut names = vec!["Ada", "Linus"];  // vec! makes one with items
names.push("Grace");                   // grow it
println!("{}", names.len());           // 3
println!("{}", names[0]);              // "Ada" (panics if out of range!)
println!("{:?}", names.get(9));        // None: the safe version
for name in &names {                   // borrow, don't consume
    println!("hi {name}");
}
```

- A `Vec` lives on the heap and grows. It must be `mut` to push.
- **`v[i]`** panics when `i` is too big. **`v.get(i)`** gives an `Option`, so you decide what happens.
- **`for x in &v`** borrows each item. `for x in v` would *move* the whole Vec.
- Functions that only read take a slice: `&[i32]`. A `&Vec<i32>` coerces into it.

- **`get(i)`** gives `Option<&T>`: a reference to the item. For plain values like numbers, **`.copied()`** turns `Option<&T>` into `Option<T>`. Looping `for n in &v` also hands you `&T`, and `total += n` works with a reference to a number.

## Your turn

- Open `src/lib.rs`.
- `podium`: push two more scores onto the list.
- `third`: use `get`. `sum_all`: loop with `for`.
