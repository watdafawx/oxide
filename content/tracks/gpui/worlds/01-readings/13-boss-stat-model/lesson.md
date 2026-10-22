Everything the bar needs is in place: kinds, labels, units, text, limits, history and the order. The bar itself will not think about any of it. It gets a list of ready pills and draws them. This boss writes the three functions that hand the bar that list.

**The snapshot.** Each second Kinetik reads every stat on the bar into a `Snapshot`, a `HashMap` from `StatKind` to `f64`. A stat it could not read has no key. `snapshot.get(&kind)` gives `Option<&f64>`, and `.copied()` turns that into the `Option<f64>` a `Reading` wants.

**One item per slot.** `bar_items` walks the order. A divider slot becomes `BarItem::Divider` unchanged. A stat slot becomes a `Pill`: its kind, its label, the reading's text and whether `is_warning` says red.

**Two lines.** Kinetik's compact layout stacks stats in pairs: the first pill on top, the next under it. A divider gets a column of its own, and a pill still waiting for a partner goes alone before it. `Option::take` hands you the waiting pill and leaves `None` behind:

```rust
Some(top) => columns.push(Column::Pills(top, Some(pill.clone()))),
```

**The hover line.** `hover_summary` asks the history for its `Summary` and writes each number with the kind's own unit, so CPU reads `min 10%` and RAM reads `min 7.9 GB`. No history yet means `no data yet`.

## Your turn

- Write `bar_items` first: the first five objectives turn green from it.
- Write `two_line_columns` with a `waiting` pill.
- Write `hover_summary`.
- Press Run to print the default bar on one line and on two.
