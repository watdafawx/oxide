The widget's graphs show 30 seconds, 1, 2 or 5 minutes, as you choose on its page. A `History` holds a number of readings, not seconds. At one reading a second, 5 minutes is 300 readings; at a half-second interval, twice that.

**Seconds to readings.** Readings per second is `1000 / interval_ms`. Multiply first, then divide, so whole numbers stay exact: 300 s × 1000 / 1000 ms is 300.

**Widen first.** `secs` is a `u32` and `interval_ms` a `u64`. Turn `secs` into a `u64` before multiplying, then the answer into a `usize`, the type `History::new` takes:

```rust
(secs as u64 * 1000 / interval_ms.max(1)) as usize
```

**Never zero.** The interval is clamped elsewhere, but a 0 here would panic with "attempt to divide by zero". `.max(1)` makes that impossible locally.

**Graphs appear.** From this quest the widget draws a small graph under each stat that has history.

## Your turn

- In `history_points`, work out the number of readings for `secs` seconds at `interval_ms`.
- Save.
