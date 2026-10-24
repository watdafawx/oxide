The bar now holds a list of sensors and a `tick` that reads them all. Something has to call `tick` later, on its own, while the window keeps drawing. Calling it from `start` directly would only ever read once, right now, before the window is even open.

**Run it later.** `cx.spawn(..)` hands GPUI a piece of async work to run on the app's own thread, between frames. It is a method of `Context<StatsBar>`, so GPUI knows which entity the work belongs to.

**An async closure.** What you hand it is `async move |this, cx| { .. }`, an **async closure**: a closure whose body is an async block. The Async world wrote `async move { .. }` blocks; this is the same thing with two arguments in front, which GPUI fills in when it runs the work.

**What GPUI passes.** `this` is a weak handle to the bar, a `WeakEntity<StatsBar>`: it names the bar without keeping it alive, and the quest Every second shows why that matters. `cx` is the `AsyncApp` from Which cx is which. `this.update(cx, |bar, cx| ..)` borrows the bar through them, like `update` on an entity, and gives back a `Result` because the bar may be gone; `.ok()` sets the error aside for now.

**Let it run.** `spawn` gives back a `Task`, and a `Task` nobody keeps is cancelled. `.detach()` lets it run to the end on its own. `main.rs` has made the same call since your first window: `on_window_closed` gives back a subscription, which also ends when dropped, and `.detach()` keeps it for the whole run. The next quest replaces `detach` with something better.

**The shape.** A spawned closure that clears the bar once, detached:

```rust
cx.spawn(async move |this, cx| {
    this.update(cx, |bar, cx| bar.clear(cx)).ok();
})
.detach();
```

**For the curious.** Async closures have been stable Rust since 1.85. Unlike a closure that returns an `async` block, the future an async closure makes may borrow the closure's arguments, which is how GPUI can lend `cx` as `&mut AsyncApp`.

## Your turn

- In `start`, spawn an async closure that ticks the bar once through `this`, with `.ok()` on the result.
- Detach the task `spawn` gives back.
- Save. Press Run: the bar reads its demo sensors once and stays there; the next quests make it repeat.
