Cloning a whole backpack just so a function can count what is inside would be silly, and moving it in would lose it. What you want is to lend it: let the function look, then carry on with your own backpack.

**A borrow.** `&bag` makes a **reference**: a pointer to `bag` that does not own it. Handing a reference to a function is called **borrowing**. The function reads through it, and at its `}` nothing is dropped, because it never owned anything.

**Say it in the type.** A parameter of type `&Backpack` means "a borrowed Backpack". Inside, `bag.items` reads the field through the reference, just as if you held the backpack yourself.

**You have done this.** The `&` in `&self` since the last world is the same thing: a method that borrows the value it is called on. So is `for item in &bag.items`, which walks a list without taking it.

```rust
fn longest_label(shelf: &Shelf) -> usize {
    let mut longest = 0;
    for label in &shelf.labels {
        if label.len() > longest {
            longest = label.len();
        }
    }
    longest
}

let shelf = Shelf { labels: vec!["jars".to_string(), "tins".to_string()] };
let n = longest_label(&shelf);       // lend it
let still_mine = shelf.labels.len(); // and keep it
```

**Many readers.** You may lend the same value to as many readers at once as you like, because nobody can change it while it is lent. That is what makes sharing safe.

**vec! and the bang.** `vec![a, b]` makes a `Vec` holding `a` and `b`. Names ending in `!`, like `format!` and `vec!`, are **macros**: shorthand that Rust expands into longer code before compiling.

## Your turn

- In `count_items`, give back the length of `bag.items`.
- In `total_chars`, loop over `&bag.items` and add each item's `len()` to a `mut` total.
