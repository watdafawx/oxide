The breadcrumb shows where a note sits: "Home > Groceries > Milk". You start at the note and follow parent links up until there is no parent. You don't know how many steps that takes, and each step gives you an `Option`.

**Loop while there is one.** `while let Some(n) = current { ... }` runs its body as long as `current` matches `Some`, with the inside named `n`, and stops at the first `None`. It is world 01's `if let`, repeated.

**Step by step.** Keep the note you are at in a variable, then end each round by moving it one step up: `current = n.borrow().parent.upgrade();`. At the root, `upgrade()` on its `Weak::new()` gives `None`, and the loop ends.

```rust
let mut levels = 0;
let mut current = folder.borrow().up.upgrade();   // start one step up
while let Some(f) = current {
    levels += 1;
    current = f.borrow().up.upgrade();
}
```

**Upside down.** Walking up gives the titles from the note to the root. The breadcrumb wants the root first, so call `.reverse()` on the `Vec` when the loop is done; it flips the items in place.

**One borrow per statement.** `n.borrow()` inside a line is dropped at that line's `;`. Each step borrows a note, reads one thing, and lets go, so the cell is free again for whatever comes next. The boss relies on this habit.

## Your turn

- In `breadcrumb`, start at the note itself, collect each title on the way up, then reverse the list.
- In `has_ancestor`, start at the note's parent and return `true` as soon as a title matches; return `false` when you run out of parents.
