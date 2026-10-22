The capture averages 118 FPS, yet it has five hitches you would feel. Kinetik shows the 1% and 0.1% lows next to FPS for exactly that: they are the speed of the worst moments.

**The slowest share.** Sort the frame times slowest first and keep the slowest 1 %: 12 of 1200 frames. Average them and turn that into FPS: `1000 / average`. The 0.1 % low uses 0.001, here one frame.

**Sorting floats.** `f64` has no plain `sort()`: `NaN` compares as neither smaller nor bigger. `total_cmp` gives every float a place. `b.total_cmp(a)` sorts the biggest, here the slowest, first:

```rust
slowest.sort_by(|a, b| b.total_cmp(a));
```

**At least one.** A short run times 0.001 rounds to 0 frames. `.max(1)` keeps one, so a 0.1 % low always exists.

**The numbers.** Here 64 for the 1 % low and 22 for the 0.1 % low, against 118 average: the gap is the stutter.

## Your turn

- In `low_fps`, copy and sort the frame times slowest first.
- Take `n` frames (the share of the count, rounded, at least 1), average them, and give back `1000.0 / average`.
- Save.
