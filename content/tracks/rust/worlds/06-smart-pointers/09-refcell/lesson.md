`Note::open` takes `&self`, because many parts of the outliner share the same note, and yet it has to add 1 to the note's open count. Put the count in a `RefCell`, and the note can change it through a shared reference.

**Put it in a cell.** Make the field `RefCell<u32>` instead of `u32`, and start it with `RefCell::new(0)`. The note itself stays shared and unchanged; only the inside of the cell changes.

**Ask for access.** `borrow()` gives a reader and `borrow_mut()` a writer. They return `Ref<T>` and `RefMut<T>`, small guard values that act like `&T` and `&mut T` through `Deref`, and that tell the cell the borrow is over when they are dropped.

**Reach the number.** `cell.borrow_mut()` is the `RefMut`, not the number, so changing the number needs the `*` from quest 03: `*cell.borrow_mut() -= 10;`. Methods need no `*`: `list.borrow_mut().push(x)` finds `push` through `Deref`.

```rust
use std::cell::RefCell;

struct Lamp {
    brightness: RefCell<u32>,
}

impl Lamp {
    fn dim(&self) {
        *self.brightness.borrow_mut() -= 10;
    }
}

let lamp = Lamp { brightness: RefCell::new(100) };
let shared = &lamp;
shared.dim();
println!("{}", lamp.brightness.borrow()); // 90
```

**How long a borrow lasts.** A `RefMut` you never put in a variable is dropped at the end of its statement, at the `;`. So each line above borrows, changes, and lets go before the next line runs.

## Your turn

- In `Note::open`, add 1 to the number inside `opens`.
- In `add_tag`, push `tag` (as a `String`) onto the list inside the `RefCell`.
