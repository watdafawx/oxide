Kinetik's General page has an update interval from 0.5 to 5 seconds. Faster costs CPU for numbers nobody can read; slower makes the bar feel dead. A settings file can hold anything, so the bar clamps what it reads.

**Milliseconds to Duration.** The setting is stored as milliseconds. `Duration::from_millis(1000)` turns that into a `Duration`, the type the timer takes.

**Clamp again.** You clamped the font size in Views. Integers have `clamp` too: below 500 gives 500, above 5000 gives 5000:

```rust
Duration::from_millis(ms.clamp(500, 5000))
```

**Applied on build.** `StatsBar::with_interval` uses `clamp_interval`, so a bar can never be built with a 1 ms timer, whatever the settings say.

**Why half a second.** Every tick reads every sensor. At 0.5 s that is already two reads a second, and the hover graph keeps 60 readings: half a minute of history.

## Your turn

- In `clamp_interval`, clamp `ms` to 500 and 5000 before `Duration::from_millis`.
- Save.
