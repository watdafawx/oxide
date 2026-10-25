Next, each note should know its parent, so a breadcrumb can show "Home > Groceries". The parent owns its children with `Rc`. If each child also owned its parent with an `Rc`, the two would keep each other alive forever.

**A cycle.** Parent and child each hold an `Rc` to the other, so each one's count includes the other's handle. Drop your own handles and both counts stop at 1, never 0. Neither value is freed, and neither `Drop` ever runs.

**A leak.** Memory that is never freed, although nothing can reach it any more, is a **memory leak**. One leaked note is a few bytes, but an editor that leaks every deleted note grows the whole time it runs. Rust's rules stop many bugs, not this one: a cycle of `Rc`s is safe, just wasteful.

**A link that does not own.** A `Weak<T>` points at a value an `Rc` owns without being an owner itself. `Rc::downgrade(&rc)` makes one: the **strong** count, the owners, stays the same, and a separate **weak** count goes up. When the strong count reaches 0 the value is freed, however many `Weak`s still point at it.

**Ask before you use.** Because the value may be gone, a `Weak` cannot be read directly. `weak.upgrade()` returns an `Option<Rc<T>>`: `Some` with a new owner while the value lives, `None` once it has been freed.

```rust
use std::rc::{Rc, Weak};

let folder = Rc::new(String::from("Projects"));
let link: Weak<String> = Rc::downgrade(&folder);

println!("{:?}", link.upgrade()); // Some("Projects")
drop(folder);
println!("{:?}", link.upgrade()); // None
```

## Your turn

- In `watch`, return a `Weak` to the note with `Rc::downgrade`.
- In `still_there`, `upgrade` the link: `Some(note)` gives back a copy of its text with `.to_string()`, `None` gives `None`.
