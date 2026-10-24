`detach` got the first tick running, but it lets go of the task for good. Soon the task loops once a second, and a detached loop has no owner to stop it. The starter now holds the task in a `let`, and throws it away.

**Tasks can vanish.** In GPUI, dropping a `Task` cancels it. `let _ = task;` drops it on the spot, so the work never runs: run the starter's test and the tick count is 0. It is the same rule as the window-closed subscription in `main.rs`: drop it, and it ends.

**Detach is forever.** `.detach()` avoided the drop by giving the task to GPUI for the whole run. For a loop that reads sensors every second, that means it keeps reading after its bar is closed, for nothing.

**Keep it alive.** Store the task in a field of the bar. It lives as long as the bar does, and when the bar is dropped, the task is dropped with it and cancelled. Holding a timer task in a field of its own looks like this:

```rust
self._timer = Some(timer_task);
```

**Why the underscore.** `_poll` is never read; it is there only to keep the task alive. The leading `_` tells Rust that is on purpose, so it does not warn about an unused field. It is an `Option` because a new bar has no task until `start` runs.

## Your turn

- In `start`, replace `let _ = task;` with storing the task in `self._poll`.
- Save. Press Run: the bar reads its demo sensors once, and now it would stop with the bar.
