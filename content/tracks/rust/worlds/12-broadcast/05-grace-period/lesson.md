The Riot stage servers learned this the hard way: on startup the data source sends a burst of recent events, and the lights flashed for a goal from ten minutes ago. Their fix, kept here: a 2-second grace period after every connect.

**Since the connect.** `Grace` remembers when the feed connected. Anything that happens less than `GRACE` after that is state catching up, not news.

**Not connected, not allowed.** Before any connection there is nothing to trust: `None` means no.

**One call.** `Option::is_some_and` checks there is a value and tests it:

```rust
self.connected_at.is_some_and(|at| now.saturating_sub(at) >= GRACE)
```

**Never negative.** `Duration` cannot be negative; `saturating_sub` stops at zero instead of panicking when `now` is somehow earlier.

## Your turn

- Write `allows`: `true` only when connected and at least `GRACE` has passed since.
- Save.
