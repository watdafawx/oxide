The widget's graphs show 30 seconds, 1, 2 or 5 minutes, as you choose on its Settings page. But a `History` holds a number of readings, not seconds. At one reading a second, 5 minutes is 300 readings; at one every half second, twice that. `history_points` does the conversion.

**Seconds to readings.** The interval is in milliseconds, so `secs` seconds is `secs * 1000` milliseconds, and that divided by `interval_ms` is the count. Multiply first, then divide, so whole numbers stay exact: 300 × 1000 / 1000 is 300, and 30 × 1000 / 500 is 60.

**Widen first.** `secs` is a `u32` and `interval_ms` a `u64`, and Rust does arithmetic only on two numbers of the same type. Turn `secs` into a `u64` with `as u64` before multiplying: a `u64` also has room for big products. At the end, `as usize` turns the answer into the type `History::new` takes.

**Never zero.** The interval is clamped elsewhere, but a 0 here would panic with "attempt to divide by zero". `.max(1)` on the divisor makes that impossible right here, where the division happens.

**The same steps, for hours.** Here is how many five-second samples fit in some hours; yours is seconds and milliseconds instead:

```rust
(hours as u64 * 3600 / sample_secs.max(1)) as usize
```

**Graphs appear.** From this quest on, the widget draws a small graph under each stat that has history.

## Your turn

- In `history_points`, work out the number of readings for `secs` seconds at one every `interval_ms`, as a `usize`.
- Save.
