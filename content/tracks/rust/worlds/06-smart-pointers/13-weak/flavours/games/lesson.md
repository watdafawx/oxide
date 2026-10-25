A power pole supplies its machines, and each machine wants to ask "which pole am I on?". The pole owns its machines with `Rc`. If each machine also owned its pole with an `Rc`, the two would keep each other alive forever.

**A cycle.** Pole and machine each hold an `Rc` to the other, so each one's count includes the other's handle. Remove your own handles and both counts stop at 1, never 0. Neither value is freed, and neither `Drop` ever runs.

**A leak.** Memory that is never freed, although nothing can reach it any more, is a **memory leak**. One leaked pole is a few bytes, but a planner that leaks every pole you delete grows the whole time it runs. Rust's rules stop many bugs, not this one: a cycle of `Rc`s is safe, just wasteful.

**A link that does not own.** A `Weak<T>` points at a value an `Rc` owns without being an owner itself. `Rc::downgrade(&rc)` makes one: the **strong** count, the owners, stays the same, and a separate **weak** count goes up. When the strong count reaches 0 the value is freed, however many `Weak`s still point at it.

**Ask before you use.** Because the value may be gone, a `Weak` cannot be read directly. `weak.upgrade()` returns an `Option<Rc<T>>`: `Some` with a new owner while the value lives, `None` once it has been freed.

```rust
use std::rc::{Rc, Weak};

let roboport = Rc::new(String::from("roboport-1"));
let link: Weak<String> = Rc::downgrade(&roboport);

println!("{:?}", link.upgrade()); // Some("roboport-1")
drop(roboport);
println!("{:?}", link.upgrade()); // None
```

## Your turn

- In `link_to`, return a `Weak` to the pole with `Rc::downgrade`.
- In `pole_still_there`, `upgrade` the link: `Some(pole)` gives back a copy of its name with `.to_string()`, `None` gives `None`.
