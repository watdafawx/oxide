FPS is "frames per second": how many frames reached the screen, over how long that took. With frame times in milliseconds, the time is their sum.

**Count over time.** 1200 frames whose times add up to 10 164.6 ms took 10.16 seconds: 1200 / 10.16 is 118 FPS. Milliseconds to seconds is `/ 1000.0`:

```rust
ms.len() as f64 / (total / 1000.0)
```

**Not averaged rates.** Averaging `1000 / ms` per frame gives a 1 ms frame the same weight as a 45 ms one, and the answer runs high. Counting frames over the real time spent is what a player sees.

**Nothing yet.** No frames means no time; give back 0 instead of dividing by zero.

**Shared by everything.** Lows, base FPS, session summaries and benchmarks all reuse this one function, so FPS means the same thing everywhere.

## Your turn

- In `average_fps`, sum the frame times and give back the count over the seconds, or `0.0` for no time.
- Save.
