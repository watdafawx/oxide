Parent owns child, child points back to parent: with two `Rc`s that is a **cycle**. Neither count ever reaches 0, so neither is freed (a leak).

The fix: the child holds a **`Weak<T>`**. It points at the parent but does not keep it alive.

```rust
use std::rc::{Rc, Weak};

let parent = Rc::new(String::from("p"));
let weak: Weak<String> = Rc::downgrade(&parent);

println!("{:?}", weak.upgrade()); // Some("p")
drop(parent);
println!("{:?}", weak.upgrade()); // None
```

- `Rc::downgrade(&rc)` -> `Weak` (adds a *weak* count, strong count unchanged).
- `weak.upgrade()` -> `Option<Rc<T>>`. `None` once every strong owner is gone.
- Rule of thumb: **owning direction = `Rc`, back-pointer = `Weak`.**

## Your turn

In `src/lib.rs`:

- `adopt(&parent, &child)`: push a clone of `child` into `parent.children`, and set `child.parent` to a weak pointer at `parent`.
- `parent_name(&node)`: upgrade the weak pointer and return the parent's name, or `None`.
