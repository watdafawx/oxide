The Riot Games stage servers learned this the hard way: on startup the data source sends a burst of recent events, and the stage lights flashed for a goal from ten minutes ago. Their fix, kept here: nothing fires for 2 seconds after every connect.

**Since the connect.** `Grace` remembers in `connected_at` when the feed connected, on the feed's clock. Anything that happens less than `GRACE` after that is old state catching up, not news.

**Not connected, not allowed.** Before any connection there is nothing to trust, so `connected_at` is `None` and the answer is no.

**Is there one, and is it right?** `Option::is_some_and` asks both questions in one call: `false` for `None`, and for `Some(value)` whatever your closure says about the value. The same call on another `Option`, the fastest goal of the night:

```rust
let record_tonight = fastest_goal.is_some_and(|seconds| seconds < 10);
```

**Never negative.** `Duration` cannot be below zero. `now.saturating_sub(at)` is the time since the connect, and it stops at zero instead of panicking if `now` is somehow earlier.

## Your turn

- In `allows`, give back `true` only when `connected_at` holds a time and at least `GRACE` has passed since it.
- Save.
