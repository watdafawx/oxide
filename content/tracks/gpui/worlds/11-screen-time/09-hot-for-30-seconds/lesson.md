Kinetik's alerts tell you when the GPU or CPU runs hot. A game loading a level spikes the GPU for a few seconds, and nobody wants a notification for that. An alert waits until the stat has stayed over its limit for 30 seconds, then says it once.

**Since when.** `over_since` remembers when each stat went over. The first reading over the limit stores `now`; later ones keep the first time, because `entry(..).or_insert(now)` only inserts when the stat is missing.

**Only once.** `alerted` holds the stats that already alerted. `HashSet::insert` gives back `true` only when the value was new, so it checks and remembers in one call:

```rust
if now.duration_since(since) >= HOT_FOR && self.alerted.insert(kind) {
```

**Back under, start again.** One reading under the limit removes the stat from both, so a spike of 10 seconds, a dip, and 10 more seconds never adds up to 30.

**Every second.** `watch_alerts` looks at the bar's readings on its own 1-second timer, with `now` from `cx.background_executor().now()`, so tests move time with `advance_clock` instead of waiting.

## Your turn

- In `HotTimer::check`, forget the stat and give back `None` when the reading is not over its limit (`is_warning`).
- Otherwise remember since when, and alert once after `HOT_FOR`. Save.
