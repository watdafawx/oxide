`Counter::hit(&self)` takes a *shared* reference but must change a number. Wrap the number in **`RefCell<T>`**.

```rust
use std::cell::RefCell;

struct Visits {
    count: RefCell<u32>,
}

impl Visits {
    fn visit(&self) {
        *self.count.borrow_mut() += 1;
    }
}

let v = Visits { count: RefCell::new(0) };
v.visit();
v.visit();
println!("{}", v.count.borrow()); // 2
```

- `borrow()` returns a `Ref<T>`, `borrow_mut()` returns a `RefMut<T>`. Both deref to the inside.
- The borrow lasts until the `Ref` / `RefMut` is dropped (end of the statement for temporaries).
- A second `borrow_mut()` while one is alive **panics**. `try_borrow_mut()` returns a `Result` so you can check.
- `result.is_err()` is true when a `Result` is an `Err` (`is_ok()` is the opposite). So `cell.try_borrow_mut().is_err()` asks "is it borrowed right now?".

- To change a number inside a `RefCell`, deref the borrow: **`*cell.borrow_mut() += 1;`**. `borrow_mut()` gives a `RefMut`, and the `*` reaches the number inside it.

## Your turn

In `src/lib.rs`:

- `Counter::hit`: increment `hits` through `&self`.
- `add_item`: push onto the list behind the `RefCell`.
- `is_locked`: return true if the cell cannot be mutably borrowed right now.
