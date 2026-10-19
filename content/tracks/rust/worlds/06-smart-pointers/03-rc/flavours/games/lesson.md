**Why now:** forty assemblers all make iron gear wheels. They must not each carry their own copy of the `Recipe`: they should **share** one. One owner per value is Rust's default, so shared ownership needs **`Rc<T>`**: reference counted.

```rust
use std::rc::Rc;

let gear = Rc::new(String::from("iron-gear-wheel"));
let asm_a = Rc::clone(&gear);            // cheap: count 1 -> 2
println!("{}", Rc::strong_count(&gear)); // 2

{
    let asm_b = Rc::clone(&gear);        // 3
}                                        // asm_b removed: back to 2
```

- `Rc::clone(&a)` does **not** copy the data. It adds an owner.
- The value is freed when the last owner is dropped.
- `Rc` gives shared **read-only** access. Changing it needs `RefCell` (coming up).
- `Rc` is single-threaded. For threads you use `Arc` (last quest).

Write `Rc::clone(&a)` rather than `a.clone()`: it tells the reader this is cheap.

## Your turn

In `src/lib.rs`:

- `share_recipe`: return a new owner of the same `Recipe`.
- `assembler_count`: return the strong count.
- `count_with_extra_assembler`: add a temporary owner in an inner block, read the count there, then read it again after the block.
