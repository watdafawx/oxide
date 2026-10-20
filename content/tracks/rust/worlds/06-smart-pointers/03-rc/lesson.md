Normally a value has one owner. When several parts of your app must keep the same data alive (shared config, a node in a graph), use **`Rc<T>`**: reference counted.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("shared"));
let b = Rc::clone(&a);               // cheap: count 1 -> 2
println!("{}", Rc::strong_count(&a)); // 2

{
    let c = Rc::clone(&a);           // 3
}                                    // c dropped: back to 2
```

- `Rc::clone(&a)` does **not** copy the data. It adds an owner.
- The value is freed when the last owner is dropped.
- `Rc` gives shared **read-only** access. Changing it needs `RefCell` (coming up).
- `Rc` is single-threaded. For threads you use `Arc` (last quest).

Write `Rc::clone(&a)` rather than `a.clone()`: it tells the reader this is cheap.

- **`Rc::ptr_eq(&a, &b)`** is true when two `Rc`s point at the *same* allocation, even if two different values would be `==`. It is how you find one specific shared item in a Vec.

## Your turn

In `src/lib.rs`:

- `share`: return a new owner of the same value.
- `owner_count`: return the strong count.
- `count_inside_and_after`: add a temporary owner in an inner block, read the count there, then read it again after the block.
