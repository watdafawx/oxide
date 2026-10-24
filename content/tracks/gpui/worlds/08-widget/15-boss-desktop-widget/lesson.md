Every part of the widget exists: its window and toggle, the header drag, snapping and the remembered place, groups, usage bars, graph length, core bars, styles, placement, click-through, and the lists of stats and headings. This boss draws the whole widget from the bar's readings and applies its window settings.

**What is given.** Everything above lives in `widget.rs`: `widget_header`, `usage_bar`, `widget_graph`, `core_bars`, `bar_share`, `parts`, `bar_stats`, `grouped`, `apply_placement`, and `WidgetView` with its fields. `render` in `lib.rs` is an empty `div()`, and `apply_widget_window` is a `todo!`.

**The names the tests look for.** The outer panel is named `widget`. Each heading is named `heading-<Group>` and shows the group's name; each stat's row is `row-<Kind>`, both written with `{:?}`, as in `format!("row-{kind:?}")`. The header, bars, graphs and core bars already name themselves.

**Read, then let go.** As the read quest before this one showed, read the bar inside a block, `let content = { let bar = self.bar.read(cx); .. };`, build the headings and rows there, and let the block give them back as a `Vec`. Use `cx` for `widget_header(cx)` only after the block.

**One row, as a closure.** Each stat's row is the same recipe: a column with the label on the left and the value on the right, then maybe a bar, then maybe a graph. Keep that recipe in a closure, `let row = |kind: StatKind| { .. };`, inside the block, and call `row(kind)` for every stat. `.justify_between()` on the label line pushes its two children to opposite ends.

**Grow it in steps.** Every builder call gives the element back, so you can keep a row in a `let mut el` and add to it only when a condition holds: `if show_bars && let Some(..) = .. { el = el.child(..); }`. Here it builds a heading with or without a count:

```rust
let mut heading = div().child(format!("{group:?}"));
if show_counts {
    heading = heading.child(format!("{}", kinds.len()));
}
```

**The panel.** After the block, the outer `div` is a column that fills the window (`.size_full()`), with a small gap and padding, rounded, on a dark background. Its children are the header, `content`, and `core_bars(&self.cores)` when there are cores.

**The window.** `apply_widget_window` applies the placement, then the lock as click-through, through the `WinApi` trait, so the same code runs against Windows and against the fake.

## Your turn

- Write `WidgetView::render`: the header and the headings first, then the rows with their bars and graphs, then the core bars. The first four objectives turn green as it fills in.
- Write `apply_widget_window`.
- Press Run: the widget opens next to the bar, ticking with real numbers.
