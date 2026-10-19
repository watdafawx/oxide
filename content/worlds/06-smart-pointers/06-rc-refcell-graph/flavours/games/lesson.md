**Why now:** a gear assembler feeds a circuit assembler that feeds a science pack assembler. You want to wire them up, tweak one machine's speed, and total the throughput of the whole line. `Rc` shares a machine but is read-only; `RefCell` changes it but has one owner. Stack them: **`Rc<RefCell<T>>`** is shared **and** mutable.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Furnace {
    rate: u32,
}

let shared = Rc::new(RefCell::new(Furnace { rate: 1 }));
let other = Rc::clone(&shared);

other.borrow_mut().rate = 10;
println!("{}", shared.borrow().rate); // 10
```

- `Rc::clone` gives another handle to the **same** `RefCell`.
- `handle.borrow()` / `handle.borrow_mut()` reach through both layers.
- Keep borrows short. Holding a `borrow_mut()` while recursing into the same machine panics.

## Your turn

In `src/lib.rs`:

- `new_assembler`: build an assembler wrapped in `Rc<RefCell<..>>` with nothing downstream.
- `connect`: push the target onto the source's `feeds`.
- `total_throughput`: this assembler's rate plus the totals of everything it feeds (recursion).
