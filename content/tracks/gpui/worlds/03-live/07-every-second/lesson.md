One read after one second is not a live bar. Kinetik reads every interval for as long as it runs. The spawn block needs to repeat: wait, read, wait, read.

**A loop that waits.** Put the timer and the update in a `loop`. Each turn awaits one interval, so the loop never spins; it runs once a second and sleeps in between.

**The handle is weak.** `this` is a `WeakEntity<StatsBar>`: it names the bar without keeping it alive, like `Weak` in Smart pointers. While the bar exists, `this.update(..)` works. Once the bar is gone, it gives back an error.

**Stop on the error.** That error is the loop's signal to end. Without the check, a loop for a closed bar would wake up every second forever:

```rust
if this.update(cx, |bar, cx| bar.tick(cx)).is_err() { break; }
```

**Belt and braces.** The task is also stored in the bar, so dropping the bar cancels it. The `break` covers the moment in between, and any loop you ever `detach`.

## Your turn

- In `start`, wrap the timer line and the update in `loop { ... }`.
- Replace `.ok();` with the `if ... is_err() { break; }`.
- Save. Press Run: the demo numbers change every second.
