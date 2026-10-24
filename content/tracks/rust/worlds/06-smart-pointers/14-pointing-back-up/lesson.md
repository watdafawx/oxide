Now build the real tree: a note owns its children with `Rc`, and each child points back up at its parent with a `Weak`. Owning goes down the tree, looking goes up. The breadcrumb asks for the parent's title through that link.

**Which way owns.** The rule of thumb: **the owning direction is `Rc`, the back-pointer is `Weak`**. A parent decides whether its children live; a child only needs to find its parent while there still is one.

**A cell for one field.** Only the list of children changes after a note is made, so only that field is a `RefCell`: `children: RefCell<Vec<Rc<Note>>>`. The title and the parent link are set once, when the note is built, so the note itself can be a plain `Rc<Note>`.

**Weak::new.** The root has no parent. `Weak::new()` makes a link that points at nothing; `upgrade()` on it always gives `None`.

```rust
let docs = Rc::new(Folder { name: "docs".to_string(), up: Weak::new() });
let file = File { name: "a.txt".to_string(), folder: Rc::downgrade(&docs) };

println!("{}", file.folder.upgrade().is_some()); // true: docs still exists
```

**Check the counts.** After `new_child`, the parent's strong count is still 1 (only your handle owns it) and its weak count is 1 (the child's link). The child's strong count is 2: your handle plus the parent's `children` list.

## Your turn

- In `new_child`, build the child with `parent: Rc::downgrade(parent)` and no children, push an `Rc::clone` of it onto the parent's `children`, and return it.
- In `parent_title`, `upgrade` the note's parent link and return a copy of the parent's title, or `None`.
