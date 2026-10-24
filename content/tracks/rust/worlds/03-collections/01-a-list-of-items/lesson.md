The shop's stock tool keeps a count for every product on the shelf. In the Foundations world you built a list with `Vec::new()` and one `push` at a time. When you already know what goes in, you can write the whole list down at once.

**Write it down.** `vec![12, 6, 1]` makes a `Vec` that already holds 12, 6 and 1, in that order. Same list as `Vec::new()` plus three pushes, in one line.

**The exclamation mark.** `vec!` ends in `!`, like `format!` and `println!`. The `!` marks a **macro**, which means a shortcut that writes ordinary code for you before the program is built. `vec![12, 6, 1]` turns into "make an empty Vec, push 12, push 6, push 1". `println!`, which you will meet in examples, prints a line of text to the screen.

**Reading the type.** The type of the list is written `Vec<u32>`, read "a Vec of u32". The angle brackets say what is inside, and every item must be that type. A list of names would be `Vec<&str>`.

```rust
let mut prices: Vec<u32> = vec![150, 400];   // a Vec of u32, two items
prices.push(90);                              // still grows: [150, 400, 90]
```

**It still grows.** A list written with `vec!` is an ordinary `Vec`. Mark it `mut` and `.push` adds to the end, as before.

## Your turn

- In `opening_stock`, write the counts 12, 6 and 1 with `vec!`.
- In `after_new_product`, take the opening stock, push 20 onto it and hand it back.
