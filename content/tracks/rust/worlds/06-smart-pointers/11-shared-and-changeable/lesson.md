The outliner is a tree: each note holds its child notes. You want to keep a handle to one note, the one you are editing, while the tree holds it too, and change it through your handle. `Rc` alone shares but cannot change; `RefCell` alone changes but has one owner. Stack them.

**Stack them.** `Rc<RefCell<Note>>` reads from the outside in: an `Rc` (several owners) of a `RefCell` (changeable inside) of a `Note`. Every `Rc::clone` is another handle to the **same** cell, so a change through one handle shows through all of them.

**Through both layers.** `switch.borrow_mut()` works on the `Rc` directly: `Rc` derefs to the `RefCell`, so the cell's methods are found through it (quest 03). The `RefMut` you get back derefs to the value, so `.brightness = 40` writes the field.

```rust
use std::cell::RefCell;
use std::rc::Rc;

struct Lamp {
    brightness: u32,
}

let lamp = Rc::new(RefCell::new(Lamp { brightness: 100 }));
let switch = Rc::clone(&lamp);

switch.borrow_mut().brightness = 40;
println!("{}", lamp.borrow().brightness); // 40
```

**A tree of them.** A note's children are a `Vec<Rc<RefCell<Note>>>`. Adding a child means borrowing the parent mutably and pushing the child's `Rc` onto that `Vec`. From then on the parent is one of the child's owners.

**A shorter name.** `Rc<RefCell<Note>>` gets long to repeat. The starter writes `type NoteRef = Rc<RefCell<Note>>;`, a **type alias**: a second, shorter name for the same type. Nothing new is created; `NoteRef` and `Rc<RefCell<Note>>` are interchangeable.

## Your turn

- In `new_note`, build a `Note` with `words` and no children, wrapped in `Rc::new(RefCell::new(...))`.
- In `add_child`, push `child` onto the parent's `children` through `borrow_mut()`.
- The third test edits a child through its own handle and reads it back through the parent. It passes once the first two do.
