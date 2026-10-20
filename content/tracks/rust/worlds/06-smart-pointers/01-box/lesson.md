Linked lists, trees and syntax trees contain themselves. Rust must know a type's size, so you put the recursive part behind a **`Box<T>`**.

A `Box` is a pointer to a value on the heap. It has a fixed size (one pointer) and it **owns** what it points to: when the Box goes out of scope, the value is freed.

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};

let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
```

Without the Box, `List` would be infinitely big and the compiler refuses it.

Matching works as usual, and `&Box<List>` auto-derefs to `&List` when you pass it on.

To walk it, match on it: the `Nil` arm returns a plain number and stops, the other arm calls the *same function* on the rest of the chain and adds its own part. A function that calls itself like this is **recursion**.

## Your turn

In `src/lib.rs` fill in:

- `prepend`: wrap the old list in a Box and put `x` in front.
- `sum`: recursion over `Cons` / `Nil`.
- `len`: same shape, counting cells.
