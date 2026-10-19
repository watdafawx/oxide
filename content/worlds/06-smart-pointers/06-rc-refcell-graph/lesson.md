`Rc` shares a value but is read-only. `RefCell` changes a value but has one owner. Stack them: **`Rc<RefCell<T>>`** is shared **and** mutable. It is how tree and graph nodes get edited from several places.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Cell2 {
    n: i32,
}

let shared = Rc::new(RefCell::new(Cell2 { n: 1 }));
let other = Rc::clone(&shared);

other.borrow_mut().n = 10;
println!("{}", shared.borrow().n); // 10
```

- `Rc::clone` gives another handle to the **same** `RefCell`.
- `handle.borrow()` / `handle.borrow_mut()` reach through both layers.
- Keep borrows short. Holding a `borrow_mut()` while recursing into the same node panics.

## Your turn

In `src/lib.rs`:

- `new_node`: build a leaf wrapped in `Rc<RefCell<..>>`.
- `add_child`: push the child onto the parent's `children`.
- `total`: value of this node plus the totals of all children (recursion).
