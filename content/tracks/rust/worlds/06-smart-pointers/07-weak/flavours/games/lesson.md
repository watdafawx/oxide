**Why now:** a power pole supplies its machines, and each machine wants to ask "which pole am I on?". If the pole owns its machines with an `Rc` and each machine owns its pole with an `Rc`, that is a **cycle**. Neither count ever reaches 0, so neither is freed (a leak).

The fix: the machine holds a **`Weak<T>`**. It points at the pole but does not keep it alive.

```rust
use std::rc::{Rc, Weak};

let pole = Rc::new(String::from("pole-north"));
let link: Weak<String> = Rc::downgrade(&pole);

println!("{:?}", link.upgrade()); // Some("pole-north")
drop(pole);
println!("{:?}", link.upgrade()); // None
```

- `Rc::downgrade(&rc)` -> `Weak` (adds a *weak* count, strong count unchanged).
- `weak.upgrade()` -> `Option<Rc<T>>`. `None` once every strong owner is gone.
- Rule of thumb: **owning direction = `Rc`, back-pointer = `Weak`.**

## Your turn

In `src/lib.rs`:

- `connect(&pole, &machine)`: push a clone of `machine` into `pole.machines`, and set `machine.pole` to a weak pointer at `pole`.
- `pole_name(&machine)`: upgrade the weak pointer and return the pole's name, or `None`.
