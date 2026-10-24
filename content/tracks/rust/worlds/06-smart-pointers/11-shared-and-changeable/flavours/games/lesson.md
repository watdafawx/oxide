A gear assembler feeds a circuit assembler, which feeds a science pack assembler. You want to keep a handle to one machine, the one you are tuning, while the line holds it too, and change its speed through your handle. `Rc` alone shares but cannot change; `RefCell` alone changes but has one owner. Stack them.

**Stack them.** `Rc<RefCell<Assembler>>` reads from the outside in: an `Rc` (several owners) of a `RefCell` (changeable inside) of an `Assembler`. Every `Rc::clone` is another handle to the **same** cell, so a change through one handle shows through all of them.

**Through both layers.** `handle.borrow_mut()` works on the `Rc` directly: `Rc` derefs to the `RefCell`, so the cell's methods are found through it (quest 03). The `RefMut` you get back derefs to the value, so `.fuel = 40` writes the field.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Furnace {
    fuel: u32,
}

let furnace = Rc::new(RefCell::new(Furnace { fuel: 100 }));
let handle = Rc::clone(&furnace);

handle.borrow_mut().fuel = 40;
println!("{}", furnace.borrow().fuel); // 40
```

**A line of them.** An assembler's `feeds` are a `Vec<Rc<RefCell<Assembler>>>`. Connecting a machine means borrowing the source mutably and pushing the target's `Rc` onto that `Vec`. From then on the source is one of the target's owners.

**A shorter name.** `Rc<RefCell<Assembler>>` gets long to repeat. The starter writes `type AssemblerRef = Rc<RefCell<Assembler>>;`, a **type alias**: a second, shorter name for the same type. Nothing new is created; the two names are interchangeable.

## Your turn

- In `new_assembler`, build an `Assembler` with `rate` and nothing in `feeds`, wrapped in `Rc::new(RefCell::new(...))`.
- In `connect`, push `target` onto the source's `feeds` through `borrow_mut()`.
- The third test retunes a machine through its own handle and reads it back through the line. It passes once the first two do.
