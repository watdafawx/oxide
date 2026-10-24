Kinetik's alerts tell you when the GPU or CPU runs hot. A game loading a level spikes the GPU for a few seconds, and nobody wants a notification for that. An alert waits until the stat has stayed over its limit for 30 seconds, then says so once.

**Back under, start again.** Check the easy case first. A reading that is not over its limit (`is_warning` from the Readings world says false) removes the stat from both `over_since` and `alerted` and gives back `None`. So a 10-second spike, a dip, and 10 more seconds never add up to 30.

**Since when.** `over_since` remembers when each stat went over. `entry(kind).or_insert(now)` stores `now` only when the stat is missing, so later readings keep the first time. It gives back a `&mut Instant`, and a `*` in front copies the time out, the same `*` as the day's counts.

**How long.** `now.duration_since(since)` is the time from `since` to `now`, a `Duration`, and it compares with `HOT_FOR` like the idle limit did.

**Only once.** `alerted` holds the stats that already alerted. `HashSet::insert` gives back `true` only when the value was not there yet, so one call both checks and remembers:

```rust
let mut warned = HashSet::new();
warned.insert(StatKind::GpuTemp); // true: new
warned.insert(StatKind::GpuTemp); // false: already there
```

**Insert last.** Put `self.alerted.insert(kind)` on the right of the `&&` after the time check. `&&` stops at the first `false`, like `||` stops at `true`, so the stat is only marked once its 30 seconds are really up.

**Every second.** `watch_alerts` looks at the bar's readings on its own 1-second timer, with `now` from `cx.background_executor().now()`. Tests move that clock with `advance_clock` instead of waiting half a minute.

## Your turn

- In `HotTimer::check`, when the reading is not over its limit, forget the stat in both and give back `None`.
- Otherwise remember since when it has been over.
- Once `HOT_FOR` has passed and the stat is new to `alerted`, give back an `Alert` with its value. Save.
