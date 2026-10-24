Many inserters drop items into one chest, and each inserter holds only a shared `&` (or an `Rc`) to it. Changing a value that others can also see is exactly what Rust normally forbids, so first see why.

**The rule.** The borrow rule from world 02: at any moment a value has either any number of `&` readers or one `&mut` writer, never both. A plain `&` cannot change what it points at.

**What it prevents.** Say you hold a `&` to the first item in a chest's `Vec`, and an inserter pushes onto that `Vec` through another reference. If the `Vec` is full, `push` moves every item to a bigger heap area and frees the old one. Your reference now points at freed memory, and reading it gives garbage or a crash. In C this bug is common; Rust rules it out at compile time.

**Move the check.** Sometimes the changes never overlap, but the compiler cannot prove it, for example on a chest an `Rc` shares. `RefCell<T>` keeps the same rule but checks it **at runtime**, while the program runs: it counts its live borrows and refuses a writer while any other borrow exists. Changing a value inside, behind a shared `&`, is called **interior mutability**.

```rust
use std::cell::RefCell;

let chest = RefCell::new(vec![String::from("iron-plate")]);
chest.borrow_mut().push(String::from("copper-plate")); // a writer, checked now
println!("{}", chest.borrow().len());                  // a reader: 2
```

**Breaking the rule.** Ask for `borrow_mut()` while another borrow is still alive and the program **panics** with `already borrowed: BorrowMutError`. The compiler's error has become a crash, so keep each borrow short. `try_borrow_mut()` returns an `Err` instead of panicking, for code that wants to check first.

**For the curious: Cell.** For a small `Copy` value such as a swing counter, `Cell<T>` is lighter: `get()` copies the value out and `set(v)` puts a new one in. It never hands out a reference, so there is nothing to count and nothing that can panic. Both cells are for one thread only; world 07 has thread-safe cousins.

## Your turn

Answer the questions, then start the next quest.
