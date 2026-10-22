Windows counts every byte an adapter ever received. A speed is how fast that count grows: the bytes since the last look, divided by the time since the last look. `NetSpeedSensor` already does the looking; you write the division.

**Two numbers per read.** `Networks::refresh` updates sysinfo's counters, and `received()` gives the bytes since the previous refresh. The sensor also remembers when it last read, with `Instant::now()`.

**Two reads needed.** On the very first read there is no previous time, so the sensor gives `None` and the bar shows `--` for one tick. `self.last.replace(now)?` stores the new time and hands back the old one, if any.

**Seconds as a fraction.** Reads are about a second apart, but never exactly. `elapsed.as_secs_f64()` gives 1.013 rather than 1, so the speed is right even when a tick runs late:

```rust
let seconds = elapsed.as_secs_f64();
```

**Divide safely.** Two reads in the same instant give 0 seconds, and dividing by 0.0 gives infinity. Guard it: no time, no speed, `0.0`.

## Your turn

- In `speed`, turn `elapsed` into seconds with `as_secs_f64()`.
- Give back `0.0` when the seconds are `0.0`, else `bytes as f64 / seconds`.
- Save. Press Run: download and upload are this PC's own.
