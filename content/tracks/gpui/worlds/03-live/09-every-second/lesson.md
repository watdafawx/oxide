One read after one second is not a live bar. Kinetik reads every interval for as long as it runs. The spawn block needs to repeat: wait, read, wait, read.

**A loop that waits.** Put the timer and the update in a `loop`. Each turn awaits one interval, so the loop never spins; it runs once a second and sleeps in between.

**The handle is weak.** `this` is the `WeakEntity<StatsBar>` from Run it later: it names the bar without keeping it alive, like `Weak` in Smart pointers. If the loop held a strong handle, the bar could never be freed while its own loop runs. While the bar exists, `this.update(..)` works; once it is gone, it gives back an error.

**Stop on the error.** That error is the loop's signal to end, so the `.ok()` that set it aside becomes a check. Without it, a loop for a closed bar would wake up every second forever. A loop that stops when a sensor stops answering has the same shape:

```rust
if sensor.read().is_none() { break; }
```

**Belt and braces.** The task is also stored in the bar, so dropping the bar cancels it. The `break` covers the moment in between, and any loop you ever `detach`.

## Your turn

- In `start`, wrap the timer line and the update in `loop { ... }`.
- Replace `.ok();` with an `if` that breaks out of the loop when the update `is_err()`.
- Save. Press Run: the demo numbers change every second.
