Kinetik's General page has an update interval from 0.5 to 5 seconds. Faster costs CPU for numbers nobody can read; slower makes the bar feel dead. A settings file can hold anything, so the bar clamps what it reads.

**Milliseconds to Duration.** The setting is stored as milliseconds. `Duration::from_millis(1000)` turns that into a `Duration`, the type the timer takes.

**Clamp again.** You clamped the font size in Views. Integers have `clamp` too: below the smallest gives the smallest, above the largest gives the largest. A setting in whole seconds, kept between 1 and 60:

```rust
Duration::from_secs(secs.clamp(1, 60))
```

**A builder of your own.** `StatsBar::with_interval` uses `clamp_interval`, so a bar can never be built with a 1 ms timer. Its signature is `fn with_interval(mut self, ms: u64) -> Self`: it takes the bar by value, changes it, and hands it back, the same shape as every link in `div()`'s chain. So `StatsBar::new(..).with_interval(1000)` reads like one.

**Why `mut self`.** Taking `self` by value makes the bar the method's own, and `mut` in front lets the method change it, just as `let mut` does for a name. The caller gave the bar away, so no one else sees it half-changed.

**Why half a second.** Every tick reads every sensor. At 0.5 s that is already two reads a second, and the hover graph keeps 60 readings: half a minute of history.

## Your turn

- In `clamp_interval`, clamp `ms` to 500 and 5000 before `Duration::from_millis`.
- Save.
