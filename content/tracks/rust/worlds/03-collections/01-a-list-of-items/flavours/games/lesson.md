Your inventory plugin keeps one stack size per hotbar slot. In the Foundations world you built a list with `Vec::new()` and one `push` at a time. A new player's kit is always the same, so you can write the whole list down at once.

**Write it down.** `vec![1, 16, 8]` makes a `Vec` that already holds 1, 16 and 8, in that order: one sword, sixteen bread, eight torches. Same list as `Vec::new()` plus three pushes, in one line.

**The exclamation mark.** `vec!` ends in `!`, like `format!` and `println!`. The `!` marks a **macro**, which means a shortcut that writes ordinary code for you before the program is built. `vec![1, 16, 8]` turns into "make an empty Vec, push 1, push 16, push 8". `println!`, which you will meet in examples, prints a line of text to the screen.

**Reading the type.** The type of the list is written `Vec<u32>`, read "a Vec of u32". The angle brackets say what is inside, and every item must be that type. A list of item names would be `Vec<&str>`.

```rust
let mut chest: Vec<u32> = vec![64, 32];   // a Vec of u32, two stacks
chest.push(5);                             // still grows: [64, 32, 5]
```

**It still grows.** A list written with `vec!` is an ordinary `Vec`. Mark it `mut` and `.push` adds to the end, as before.

## Your turn

- In `starter_kit`, write the stack sizes 1, 16 and 8 with `vec!`.
- In `kit_after_pickup`, take the starter kit, push 64 (a stack of cobblestone) and hand it back.
