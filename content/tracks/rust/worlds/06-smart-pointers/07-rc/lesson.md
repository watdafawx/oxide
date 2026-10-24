A note can show up in two outlines at once: "Groceries" is linked from both "Home" and "This week". Neither outline is the one true owner, and the note must stay alive as long as either still shows it. Rust's one-owner rule has no room for that, so you need a different kind of owner.

**Try the obvious thing.** Give the note to both: `let home = vec![note]; let week = vec![note];`. Rust stops at the second line with `error[E0382]: use of moved value: note`. The first `vec!` took ownership, and a value has exactly one owner.

**Count the owners.** `Rc<T>`, short for **reference counted**, puts the value on the heap next to a counter of how many owners it has. `Rc::clone(&a)` makes one more owner of the *same* value and adds 1 to the counter; it never copies the value. Each owner that is dropped takes 1 away, and when the counter reaches 0 the value is freed.

```rust
use std::rc::Rc;

let a = Rc::new(String::from("Home"));
let b = Rc::clone(&a);                // count 1 -> 2
println!("{}", Rc::strong_count(&a)); // 2

{
    let c = Rc::clone(&a);            // 3
}                                     // c dropped: back to 2
```

**Write Rc::clone.** `a.clone()` compiles too, but on most types `clone` means "copy everything". `Rc::clone(&a)` tells the reader at a glance that this is the cheap kind. `Rc::strong_count` reads the counter; quest 13 explains the word "strong".

**Read-only, one thread.** Several owners may be reading at once, so an `Rc` never lets you change the value inside; quest 09 shows how to do that safely. Its counter is also not safe to update from two threads at the same time. World 07 has `Arc`, the thread-safe version.

## Your turn

- In `share`, return a new owner of the same note with `Rc::clone`.
- In `owner_count`, return the strong count.
- In `count_inside_and_after`, make an extra owner inside a `{ }` block and read the count there, then read it again after the block.
