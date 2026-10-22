Every part of the widget exists: its window and toggle, the header drag, snapping and the remembered place, groups, usage bars, graph length, core bars, styles, placement and click-through. This boss draws the widget from scratch and applies its window settings.

**What is given.** All the pieces above, `WidgetView` and its fields, `widget_graph`, `bar_share`, and the Widget settings inside `Settings`. `render` is empty.

**Read once.** `self.bar.read(cx)` borrows the bar for the whole frame: its order, snapshot, texts and histories. Build everything that needs the bar inside one block, then let the borrow end before `cx` is used again.

**One stat, then groups.** A closure for one stat's row keeps the group loop short: label and value, then a bar and a graph as the style says. The groups are `GROUPS` in order, each with its stats from `group_of`, empty ones skipped:

```rust
let (show_bars, show_graphs) = parts(cx.global::<Settings>().widget.style);
```

**The window.** `apply_widget_window` applies placement and lock through the `WinApi` trait, so the same code runs against Windows and against the fake.

## Your turn

- Write `WidgetView::render`: the first four objectives turn green as it fills in.
- Write `apply_widget_window`.
- Press Run: the widget opens next to the bar, ticking with real numbers.
