Every part of a live bar has been built once: state in an entity, a spawned loop on a timer, a weak handle that ends it, a clamped interval, histories, and redrawing only on change. This boss writes them again in one place, and the bar runs on this PC's real CPU and RAM.

**What is given.** The sensors (`real.rs`, `sensor.rs`, `live_sensors`), the drawing (`view.rs`), `clamp_interval` and the `StatsBar` struct are written. `new` builds a bar that shows `--` everywhere until the first tick.

**Start the loop.** `start` spawns the loop: wait one interval, tick, repeat; break when the bar is gone; keep the task in `self._poll` so dropping the bar cancels it.

**One tick.** `tick` reads every sensor once. A value goes into the snapshot and the stat's history; `None` takes the stat out of the snapshot. Then the items are rebuilt and, only if they differ, stored and notified:

```rust
if items != self.items {
```

**The interval.** `with_interval(ms)` applies the setting through `clamp_interval`, so 10 ms becomes 500 ms.

## Your turn

- Write `start`, then `tick`: the first objectives turn green as they work.
- Write `with_interval`.
- Press Run: real CPU and RAM tick every second, and the window closes cleanly.
