The overlay's frame-time graph shows the last 3 seconds as a line; a stutter is a spike. Kinetik's README: "spikes are stutters". To mark them, the code needs a rule for "much slower than usual".

**Usual is the median.** The median is the middle frame time once sorted. Unlike the average, a few huge frames do not move it: it says what a normal frame is right now.

**Twice the median.** A frame over twice the median is a spike. Here the median is 8.4 ms, so anything over 16.8 ms counts: five frames in the capture.

**Even counts.** With an even number of frames there are two middles; the median is their average. `is_multiple_of(2)` asks "is it even?":

```rust
let median = if sorted.len().is_multiple_of(2) { (sorted[mid - 1] + sorted[mid]) / 2.0 } else { sorted[mid] };
```

**Positions, not times.** The graph needs to know where to draw each red tick, so `spikes` gives the positions of the slow frames in the run.

## Your turn

- In `spikes`, sort a copy and work out the median.
- Give back the positions of frames slower than twice the median.
- Save.
