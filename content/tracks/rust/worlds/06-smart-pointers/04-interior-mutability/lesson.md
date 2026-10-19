Rust's rule is "many readers or one writer". **Interior mutability** lets you change a value through a shared `&` reference. The rule is not dropped: it is checked **at runtime** instead of compile time.

```rust
use std::cell::{Cell, RefCell};

let hits = Cell::new(0);
hits.set(hits.get() + 1);          // copy in, copy out

let names = RefCell::new(vec![String::from("ann")]);
names.borrow_mut().push(String::from("bo")); // &mut checked now
println!("{}", names.borrow().len());        // 2
```

- **`Cell<T>`**: for small `Copy` values. `get` / `set`, never a reference inside.
- **`RefCell<T>`**: `borrow()` gives a `Ref` (shared), `borrow_mut()` gives a `RefMut` (exclusive).
- Break the rule (a `borrow_mut()` while any borrow is alive) and the program **panics**. `try_borrow_mut()` returns `Err` instead.
- Both are single-threaded. They are usually paired with `Rc`: `Rc<RefCell<T>>` = shared **and** changeable.

Read the two questions below, then continue.
