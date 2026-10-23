The screen time page opens with a few lines about the day: how long in total, when you started and stopped, which program took the most time, and which hour was busiest.

**Most of something.** `max_by_key` walks an iterator and keeps the item with the biggest key. Over the map's `(name, seconds)` pairs, the key is the seconds:

```rust
day.apps.iter().max_by_key(|(_, secs)| **secs).map(|(name, _)| name.clone())
```

**Nothing is an answer.** On an empty day there is no most used program, and `max_by_key` gives `None` for an empty iterator. The page shows `--`.

**Busiest hour.** The hours are indexes 0 to 23. Keep the ones with any seconds, then take the one with the most. Without the filter, an empty day would pick an hour of zeros.

**Already counted.** Total is the sum of every program; first and last use were kept by `Day::add` as seconds after midnight. `clock` writes them as `08:00`.

## Your turn

- In `summarize`, fill `most_used` with the program that has the most seconds.
- Fill `busiest_hour` with the hour that has the most seconds, skipping empty hours.
- Save.
