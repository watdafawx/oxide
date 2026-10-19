**Why now:** many inserters will drop items into one chest, and every inserter only holds a shared `&` to it. Rust's rule is "many readers or one writer", so how does a shared handle change the chest? **Interior mutability**: the rule is not dropped, it is checked **at runtime** instead of compile time.

```rust
use std::cell::{Cell, RefCell};

let swings = Cell::new(0);
swings.set(swings.get() + 1);      // copy in, copy out

let chest = RefCell::new(vec![String::from("iron-plate")]);
chest.borrow_mut().push(String::from("copper-plate")); // &mut checked now
println!("{}", chest.borrow().len());                  // 2
```

- **`Cell<T>`**: for small `Copy` values (a swing counter). `get` / `set`, never a reference inside.
- **`RefCell<T>`**: `borrow()` gives a `Ref` (shared), `borrow_mut()` gives a `RefMut` (exclusive). Good for a whole inventory.
- Break the rule (a `borrow_mut()` while any borrow is alive) and the program **panics**. `try_borrow_mut()` returns `Err` instead.
- Both are single-threaded. They are usually paired with `Rc`: `Rc<RefCell<T>>` = shared **and** changeable.

Read the two questions below, then continue.
