The overlay's frame-time graph shows the last 3 seconds as a line, and a stutter is a spike in it. Kinetik's README puts it plainly: "spikes are stutters". To mark them in red, the code needs a rule for "much slower than usual".

**Usual is the median.** The **median** is the middle frame time once they are sorted. Unlike the average, a few huge frames hardly move it: the spikes themselves would drag an average up, but the middle of the sorted list stays put. So it says what a normal frame is right now.

**Twice the median.** A frame that took more than twice the median is a spike. Here the median is 8.4 ms, so anything over 16.8 ms counts: five frames in the capture.

**Even counts.** Sort a copy, as for the lows, and take the item at `len / 2`. With an even number of frames there are two middles, at `mid - 1` and `mid`, and the median is their average. `len.is_multiple_of(2)` asks "is it even?". With no frames there is no median; give back an empty list.

**Positions, not times.** The graph needs to know where to draw each red tick, so `spikes` gives the positions of the slow frames, not their times. `enumerate()` pairs each frame time with its position, `filter` keeps the slow pairs, and `map(|(i, _)| i)` keeps just the position. Here it is with a fixed 20 ms limit instead of the median:

```rust
ms.iter().enumerate().filter(|(_, m)| **m > 20.0).map(|(i, _)| i).collect()
```

**Two stars.** `iter()` lends each frame time, so each pair is `(position, &f64)`, and `filter` lends the pair again. The `m` it unpacks is two references deep, a `&&f64`, and `**m` steps through both to the number. The Rust track's [Follow the arrow](/quest/rust/02-ownership/09-follow-the-arrow) names this `**`.

**For the curious.** `sort_by(f64::total_cmp)` is the same sort as `sort_by(|a, b| a.total_cmp(b))`: a method's path can stand in for a closure that only calls it.

## Your turn

- In `spikes`, give back an empty list for no frames. Otherwise sort a copy and work out the median.
- Give back the positions of the frames slower than twice the median.
- Save.
