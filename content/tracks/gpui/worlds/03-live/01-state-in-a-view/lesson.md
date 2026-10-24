Until now the bar drew a fixed demo snapshot. A live bar gets new numbers every second, so it has to hold its numbers somewhere it can change them. The place for that is the view's own fields.

**State lives in fields.** A view is a struct GPUI keeps between frames, and its fields are its **state**: what it knows right now. `StatsBar` now has `snapshot`, the latest readings, and `items`, what the bar draws.

**Render only reads.** `render`, now in `view.rs`, draws from `self.items` and `self.look` and never computes numbers itself. Each frame's elements come fresh from the fields, as in What a view is, so changing the fields is how the picture changes.

**Fill them once.** `StatsBar::new(snapshot)` makes a bar from a snapshot. It keeps the snapshot and turns it into items right away, with your `bar_items` from Readings. Keeping a raw number and what it looks like side by side has this shape:

```rust
let text = format_percent(cpu);
```

**Why keep both.** The snapshot is the raw numbers; the items are what they look like. Later quests change the snapshot every second and rebuild the items from it.

**Drawing stays put.** All the drawing from Views moved into `view.rs` unchanged. This world only touches how the numbers get into the bar.

## Your turn

- In `StatsBar::new`, replace the empty `items` with the items `bar_items` makes from `snapshot`, the default order and the default limits.
- Save. Press Run: the demo bar is back, now drawn from the bar's own state.
