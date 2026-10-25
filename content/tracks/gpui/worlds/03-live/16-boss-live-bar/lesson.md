Every part of a live bar has been built once: state in an entity, a spawned loop on a timer, a weak handle that ends it, a clamped interval, histories, and redrawing only on change. This boss writes them again in one place, and the bar runs on your PC's real CPU and RAM.

**What is given.** The sensors (`real.rs`, `sensor.rs`, `live_sensors`), the drawing (`view.rs`), `clamp_interval` and the `StatsBar` struct are written. `new` builds a bar that shows `--` everywhere until the first tick.

**Start the loop.** `start` spawns the loop with `cx.spawn` and an async closure: wait one interval, tick through `this`, repeat; break when `this` says the bar is gone; keep the task in `self._poll` so dropping the bar cancels it.

**One tick.** `tick` reads every sensor once. A value goes into the snapshot and the stat's history; `None` takes the stat out of the snapshot. Then the items are rebuilt and, only if they differ, stored and notified.

**The interval.** `with_interval(ms)` is the builder from Update interval: it takes the bar by value, applies the setting through `clamp_interval`, so 10 ms becomes 500 ms, and hands the bar back.

**The shape of a tick.** Four steps, in this order, so the count goes up even when nothing is redrawn:

```rust
for sensor in &mut self.sensors { /* snapshot, history */ }
self.ticks += 1;
let items = /* rebuilt from the snapshot */;
// store and notify only if items changed
```

## Your turn

- Write `start`, then `tick`: the first objectives turn green as they work.
- Write `with_interval`.
- Press Run: real CPU and RAM tick every second, and the window closes cleanly.
