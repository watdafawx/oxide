Until now the bar drew a fixed demo snapshot. A live bar gets new numbers every second, so it has to hold its numbers somewhere it can change them. The place for that is the view's own fields.

**State lives in fields.** `StatsBar` now has `snapshot`, the latest readings, and `items`, what the bar draws. `render` in `view.rs` reads `self.items` and `self.look`; it never computes numbers itself.

**Fill them once.** `StatsBar::new(snapshot)` makes a bar from a snapshot. It keeps the snapshot and turns it into items with your `bar_items` from Readings, right away:

```rust
items: bar_items(&default_order(), &snapshot, &Limits::default()),
```

**Why keep both.** The snapshot is the raw numbers; the items are what they look like. Later quests change the snapshot every second and rebuild the items from it.

**Drawing stays put.** All the drawing from Views moved into `view.rs` unchanged. This world only touches how the numbers get into the bar.

## Your turn

- In `StatsBar::new`, change `items: Vec::new()` to build the items from `snapshot` with `bar_items`.
- Save. Press Run: the demo bar is back, now drawn from the bar's own state.
