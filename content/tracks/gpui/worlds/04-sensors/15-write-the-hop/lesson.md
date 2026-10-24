The hop works, but it was written for you. It is the pattern for any slow job in a GPUI app, so write it yourself once. In this quest's `start`, the loop copies the sensors and the wanted kinds out of the bar, and then stops short.

**What you have.** Inside `cx.spawn(async move |this, cx| ..)`: `this` is a weak handle to the bar, `cx` is the async context, and after `read_with` you hold `sensors`, a clone of the `Arc`, and `wanted`, a `HashSet`. Both are Send, so both may go to a worker.

**Out to a worker.** `cx.background_executor().spawn(async move { .. })` runs the block on a worker thread. `move` hands `sensors` and `wanted` over to it, which is what lets the block outlive this loop round. Inside, call `read_sensors(&sensors, &wanted)`; the block's last value is the task's result.

**Wait for it.** `spawn` gives back a `Task<Vec<SensorRead>>` straight away, before the reads are done. `.await` on it pauses this foreground task, not the UI thread, and gives you the reads once the worker is finished.

**Back on the UI thread.** `this.update(cx, |bar, cx| ..)` runs a closure with the bar itself and its `Context`, on the UI thread, where `apply` stores the reads and calls `notify`. It fails when the bar is gone, and then the loop must `break`. The same two steps, saving a settings file:

```rust
let saved = cx.background_executor().spawn(async move { write_settings(&path, &text) }).await;
if this.update(cx, |page, cx| page.show_saved(saved, cx)).is_err() { break; }
```

## Your turn

- In `start`, run `read_sensors` on the background executor and wait for its reads.
- Hand the reads to `apply` with `this.update`, and leave the loop when that fails.
- Save. Press Run: the bar ticks, and ping never freezes it.
