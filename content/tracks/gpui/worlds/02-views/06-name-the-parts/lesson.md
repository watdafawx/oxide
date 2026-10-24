The value is about to move around: it gets a fixed width, then a second line. Tests and later quests need to find the label and the value on their own, so each one gets a box and a name. You have seen the line that names a pill; now you write two like it.

**Names for tests.** As in The first pill, `.debug_selector(..)` names a box so a test can look it up with `debug_bounds(..)` and find where it was drawn. It changes nothing on screen.

**Only when testing.** Its closure is a recipe for the name, and GPUI runs it only when its test tools are switched on. That is why the name can be built with `format!` for free: in a plain release build the closure never runs, and no text is ever made.

**One name per kind.** `format!("pill-{kind:?}")` writes the kind with `{:?}`, the Debug format from the Rust track, so `StatKind::CpuTemp` gives `pill-CpuTemp`. Every kind gets its own name with no `match`.

**Why copy the kind.** The braces inside `format!` take a plain name, not a path, so `{pill.kind:?}` does not compile. That is why `pill_element` starts with `let kind = pill.kind;`. `StatKind` is `Copy`, so the copy is free.

**Why `move`.** `move` makes the closure take its own copy of `kind` instead of borrowing it. This closure runs at once, so it would work without; but closures GPUI keeps for later, like the click handlers in the Interaction world, must own everything they use. The bar writes `move` on every name closure, so the habit is already there.

**A named box.** Wrapping content in a named box is one `div()` inside a `.child(..)`. An icon box named after the kind would look like this:

```rust
.child(div().debug_selector(move || format!("icon-{kind:?}")).child(icon))
```

## Your turn

- In `pill_element`, replace the two plain children with two named boxes: `label-<Kind>` holding the label, then `value-<Kind>` holding the value text.
- Save. Press Run: the bar looks the same; the test now finds both parts of every pill.
