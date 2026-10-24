`Inserter::swing` takes `&self`, because many machines point at the same inserter, and yet it has to add 1 to its swing count. A chest's inventory has the same problem. Put the data in a `RefCell`, and it can change through a shared reference.

**Put it in a cell.** Make the field `RefCell<u32>` instead of `u32`, and start it with `RefCell::new(0)`. The inserter itself stays shared and unchanged; only the inside of the cell changes.

**Ask for access.** `borrow()` gives a reader and `borrow_mut()` a writer. They return `Ref<T>` and `RefMut<T>`, small guard values that act like `&T` and `&mut T` through `Deref`, and that tell the cell the borrow is over when they are dropped.

**Reach the number.** `cell.borrow_mut()` is the `RefMut`, not the number, so changing the number needs the `*` from quest 03: `*cell.borrow_mut() -= 10;`. Methods need no `*`: `chest.borrow_mut().push(x)` finds `push` through `Deref`.

```rust
use std::cell::RefCell;

struct Furnace {
    fuel: RefCell<u32>,
}

impl Furnace {
    fn burn(&self) {
        *self.fuel.borrow_mut() -= 10;
    }
}

let furnace = Furnace { fuel: RefCell::new(100) };
let shared = &furnace;
shared.burn();
println!("{}", furnace.fuel.borrow()); // 90
```

**How long a borrow lasts.** A `RefMut` you never put in a variable is dropped at the end of its statement, at the `;`. So each line above borrows, changes, and lets go before the next line runs.

## Your turn

- In `Inserter::swing`, add 1 to the number inside `swings`.
- In `put_in_chest`, push `item` (as a `String`) onto the inventory inside the `RefCell`.
