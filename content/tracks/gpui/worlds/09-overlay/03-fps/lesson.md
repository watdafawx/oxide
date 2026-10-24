FPS is "frames per second": how many frames reached the screen, over how long that took. With frame times in milliseconds, how long it took is simply their sum.

**Count over time.** 1200 frames whose times add up to 10 164.6 ms took 10.16 seconds, and 1200 / 10.16 is 118 FPS. `ms.iter().sum()` adds the frame times; the `let total: f64` in front tells `sum` which type to add up in. Milliseconds become seconds with `/ 1000.0`.

**Count as a decimal.** `ms.len()` is a `usize`, a whole number. Rust does not divide a `usize` by an `f64`, so turn the count into one first with `as f64`.

**Not averaged rates.** The tempting shortcut is to work out `1000 / ms` for every frame and average those. That gives a 1 ms frame the same weight as a 45 ms one, although the 45 ms frame was on screen 45 times longer, and the answer runs high. Counting frames over the real time spent is what a player sees.

**The same sum, for a download.** Here the same steps give megabytes per second from the milliseconds each piece took:

```rust
let total_ms: f64 = piece_ms.iter().sum();
let mb_per_second = megabytes / (total_ms / 1000.0);
```

**Nothing yet.** No frames means no time. Give back `0.0` instead of dividing by zero, which for floats gives `NaN`, "not a number", not a crash, and would show up on screen as nonsense.

**Shared by everything.** Lows, base FPS, session summaries and benchmarks all reuse this one function, so FPS means the same thing everywhere.

## Your turn

- In `average_fps`, sum the frame times into a `total`.
- Give back the count of frames over the seconds, or `0.0` when the total is zero.
- Save.
