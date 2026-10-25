Forty assemblers all make iron gear wheels. They must not each carry their own copy of the `Recipe`: they should share one, and it must stay alive as long as any assembler still uses it. Rust's one-owner rule has no room for that, so you need a different kind of owner.

**Try the obvious thing.** Give the recipe to two assemblers: `let a = Assembler { recipe }; let b = Assembler { recipe };`. Rust stops at the second line with `error[E0382]: use of moved value: recipe`. The first assembler took ownership, and a value has exactly one owner.

**Count the owners.** `Rc<T>`, short for **reference counted**, puts the value on the heap next to a counter of how many owners it has. `Rc::clone(&a)` makes one more owner of the *same* value and adds 1 to the counter; it never copies the value. Each owner that is dropped takes 1 away, and when the counter reaches 0 the value is freed.

```rust
use std::rc::Rc;

let gear = Rc::new(String::from("iron-gear-wheel"));
let asm_a = Rc::clone(&gear);            // count 1 -> 2
println!("{}", Rc::strong_count(&gear)); // 2

{
    let asm_b = Rc::clone(&gear);        // 3
}                                        // asm_b removed: back to 2
```

**Write Rc::clone.** `gear.clone()` compiles too, but on most types `clone` means "copy everything". `Rc::clone(&gear)` tells the reader at a glance that this is the cheap kind. `Rc::strong_count` reads the counter; quest 13 explains the word "strong".

**Read-only, one thread.** Many assemblers may be reading at once, so an `Rc` never lets you change the value inside; quest 09 shows how to do that safely. Its counter is also not safe to update from two threads at the same time. World 07 has `Arc`, the thread-safe version.

## Your turn

- In `share_recipe`, return a new owner of the same recipe with `Rc::clone`.
- In `assembler_count`, return the strong count.
- In `count_with_extra_assembler`, make an extra owner inside a `{ }` block and read the count there, then read it again after the block.
