The bar now holds a list of sensors and a `tick` that reads them all. Something has to call `tick` later, on its own, while the window keeps drawing. Calling it from `start` directly would only ever read once, right now.

**Run it later.** `cx.spawn(async move |this, cx| { ... })` starts an async block that GPUI runs on the app's own thread, between frames. `this` is a handle to the bar, and `this.update(cx, |bar, cx| bar.tick(cx))` borrows it, like `update` on an entity.

**Tasks can vanish.** `spawn` gives back a `Task`. In GPUI, dropping a `Task` cancels it. The starter writes `let _ = task;`, which drops it on the spot, so the work never runs.

**Keep it alive.** Store the task in the bar's `_poll` field; the underscore tells Rust it is kept on purpose and never read. It lives as long as the bar does, and goes away with it:

```rust
self._poll = Some(task);
```

**Async from Rust.** This is the `async` and `.await` from the Async world, with GPUI as the runtime instead of tokio.

## Your turn

- In `start`, replace `let _ = task;` with `self._poll = Some(task);`.
- Save. Press Run: the bar reads its demo sensors once and stays there.
