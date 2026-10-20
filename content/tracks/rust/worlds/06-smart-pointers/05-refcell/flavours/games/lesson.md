**Why now:** `Inserter::swing(&self)` takes a *shared* reference, because many things point at the same inserter, but it must still count a swing. A chest's inventory has the same problem. Wrap the data in **`RefCell<T>`**.

```rust
use std::cell::RefCell;

struct Chest {
    slots: RefCell<Vec<String>>,
}

impl Chest {
    fn put(&self, item: &str) {
        self.slots.borrow_mut().push(item.to_string());
    }
}

let chest = Chest { slots: RefCell::new(Vec::new()) };
chest.put("iron-plate");
chest.put("copper-plate");
println!("{}", chest.slots.borrow().len()); // 2
```

- `borrow()` returns a `Ref<T>`, `borrow_mut()` returns a `RefMut<T>`. Both deref to the inside.
- The borrow lasts until the `Ref` / `RefMut` is dropped (end of the statement for temporaries).
- A second `borrow_mut()` while one is alive **panics**. `try_borrow_mut()` returns a `Result` so you can check.
- `result.is_err()` is true when a `Result` is an `Err` (`is_ok()` is the opposite). So `cell.try_borrow_mut().is_err()` asks "is it borrowed right now?".

- To change a number inside a `RefCell`, deref the borrow: **`*cell.borrow_mut() += 1;`**. `borrow_mut()` gives a `RefMut`, and the `*` reaches the number inside it.

## Your turn

In `src/lib.rs`:

- `Inserter::swing`: increment `swings` through `&self`.
- `put_in_chest`: push onto the inventory behind the `RefCell`.
- `chest_busy`: return true if the chest cannot be mutably borrowed right now.
