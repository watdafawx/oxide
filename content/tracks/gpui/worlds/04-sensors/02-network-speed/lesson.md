Windows counts every byte an adapter ever received. A speed is how fast that count grows: the bytes since the last look, divided by the time since the last look. `NetSpeedSensor` already does the looking; you write the division.

**Two numbers per read.** `Networks::refresh` updates sysinfo's counters, and `received()` gives the bytes since the previous refresh. The sensor also notes the moment it read, with `Instant::now()`. An `Instant` is a point in time; subtracting two of them gives a `Duration`, the time between.

**Two reads needed.** On the very first read there is no previous moment, so the sensor gives `None` and the bar shows `--` for one tick. `self.last.replace(now)?` stores the new moment and hands back the old one, if any.

**Seconds as a fraction.** Reads are about a second apart, but never exactly. `as_secs()` would round 1.013 s down to 1. `as_secs_f64()` keeps the fraction, so the speed stays right when a tick runs late.

**Divide safely.** Two reads in the same instant give 0 seconds, and dividing by `0.0` gives infinity, which the bar would print as `inf`. Guard it: no time, no speed, `0.0`. The same shape, for songs played per minute:

```rust
let minutes = played.as_secs_f64() / 60.0;
let per_minute = if minutes == 0.0 { 0.0 } else { songs as f64 / minutes };
```

## Your turn

- In `speed`, turn `elapsed` into seconds with the fraction kept.
- Give back `0.0` when there were no seconds, else the bytes (as `f64`) per second.
- Save. Press Run: download and upload are your PC's own.
