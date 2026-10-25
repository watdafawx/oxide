Everything the bar needs is in place: kinds, labels, units, text, limits, history, the order, the two-line pairing and the hover line. The bar itself will not think about any of it. It gets a list of ready pills and draws them. This boss writes the function that turns the bar's order and this second's readings into that list.

**The snapshot.** Each second Kinetik reads every stat on the bar into a `Snapshot`. `type Snapshot = HashMap<StatKind, f64>;` is a **type alias**: a short name for a long type, the same type under a second name, so every signature says what the map means. A stat Kinetik could not read has no key.

**From key to reading.** `snapshot.get(&kind)` gives an `Option<&f64>`: a borrowed number, or `None` for a missing key. `.copied()` turns it into the `Option<f64>` a `Reading` wants, as `.get(i).copied()` did for lists in the Collections world.

**One item per slot.** `bar_items` walks the order and gives back one `BarItem` for each slot, in the same order. A divider slot becomes the same divider. A stat slot becomes a `Pill`: its kind, its label, its reading's text, and whether `is_warning` says red.

**Borrowed kinds.** `for slot in order` lends each slot, so in `Slot::Stat(kind)` the `kind` is a `&StatKind`. `Reading` and `Pill` want the kind itself: `*kind` copies it out, because `StatKind` is `Copy`. Methods need no `*`: `kind.label()` works on the reference.

**The shape.** One loop and one `match`, with an arm per kind of slot:

```rust
for slot in order {
    match slot {
        Slot::Stat(kind) => { /* a reading, then a pill */ }
        Slot::Divider(style) => { /* the same divider */ }
    }
}
```

## Your turn

- Write `bar_items`: one item per slot, a `Pill` for each stat and the same divider for each divider.
- The first five objectives turn green from it; the last checks it together with `two_line_columns`.
- Press Run to print the default bar on one line and on two.
