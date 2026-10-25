Kinetik's compact layout stacks stats in pairs, two short lines in the taskbar's height: the first pill on top, the next one under it. A divider gets a column of its own. Walking the bar's items, the code has to hold on to one pill while it waits for a partner.

**What the bar gets.** A `Pill` is one stat, ready to draw: its kind, label, text and whether it is red. Its `label` is a `&'static str`, text baked into the program that lasts the whole run, so a pill can hold it without owning a copy. A `BarItem` is a pill or a divider, and a `Column` holds one or two pills, or a divider.

**A pill that waits.** `waiting: Option<Pill>` is the memory: `None` when no pill waits, `Some(pill)` while one sits on top, waiting for a partner.

**Try the obvious thing.** `match waiting { Some(top) => .. }` moves the pill out of `waiting` into `top`. That works once. On the next pill the loop reads `waiting` again, and Rust refuses, because its value is gone:

```text
error[E0382]: use of moved value
```

**The word for it.** `Option::take` hands you what was inside and leaves `None` behind, in one step. `waiting` stays a usable variable, empty and ready to fill. It changes `waiting`, so `waiting` is `let mut`. An alert that should show only once works the same way:

```rust
let mut alert = Some("CPU hot".to_string());
let shown = alert.take(); // Some("CPU hot")
// alert is now None, so it cannot show twice
```

**Three moments.** A pill arrives: with one waiting, the two make a column; with none, this one waits. A divider arrives: a waiting pill goes alone first, then the divider. The items end: a pill still waiting goes alone.

**Copies to keep.** `items` is borrowed, so the loop sees each `pill` as a `&Pill`, and a column owns its pills: `pill.clone()` makes the copy it keeps. The divider's `*style` copies the style out, because `DividerStyle` is `Copy`.

## Your turn

- In `two_line_columns`, fill the pill arm: take the waiting pill; with one, push a column of the two; without one, make this pill wait.
- In the divider arm, push a waiting pill alone before the divider.
- After the loop, push the pill that is still waiting, if there is one.
