The tech tree shows what you must research before a tech: "Automation > Logistics > Fast inserter". You start at the tech and follow prerequisite links up until there is none. You don't know how many steps that takes, and each step gives you an `Option`.

**Loop while there is one.** `while let Some(t) = current { ... }` runs its body as long as `current` matches `Some`, with the inside named `t`, and stops at the first `None`. It is world 01's `if let`, repeated.

**Step by step.** Keep the tech you are at in a variable, then end each round by moving it one step up: `current = t.borrow().prerequisite.upgrade();`. At the starting tech, `upgrade()` on its `Weak::new()` gives `None`, and the loop ends.

```rust
let mut hops = 0;
let mut current = belt.borrow().upstream.upgrade();   // start one step up
while let Some(b) = current {
    hops += 1;
    current = b.borrow().upstream.upgrade();
}
```

**Upside down.** Walking up gives the names from the tech to the start. The research path wants the start first, so call `.reverse()` on the `Vec` when the loop is done; it flips the items in place.

**One borrow per statement.** `t.borrow()` inside a line is dropped at that line's `;`. Each step borrows a tech, reads one thing, and lets go, so the cell is free again for whatever comes next. The boss relies on this habit.

## Your turn

- In `research_path`, start at the tech itself, collect each name on the way up, then reverse the list.
- In `needs`, start at the tech's prerequisite and return `true` as soon as a name matches; return `false` when you run out of prerequisites.
