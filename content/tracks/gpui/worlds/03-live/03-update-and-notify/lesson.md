A new CPU reading arrives. Changing the snapshot and the items is not enough: GPUI does not watch your fields, and it draws only when something tells it to. Without that signal, the numbers change in memory and the screen keeps showing the old ones.

**Change through the app.** `bar.update(cx, |bar, cx| ...)` lends a `&mut StatsBar` and a `Context<StatsBar>` for the closure. The context is how the bar talks back to GPUI while it changes.

**Then say so.** `cx.notify()` tells GPUI "this entity changed": its window draws again on the next frame, and anything watching the bar hears about it:

```rust
cx.notify();
```

**Forget it once.** Run the starter's test: the text is right, but nothing was notified. That is the classic GPUI bug: the state is new and the screen is stale.

**Notify, don't draw.** You never call `render` yourself. Notify is cheap; GPUI collects all the notifies of one frame and draws once.

## Your turn

- At the end of `set_reading`, add `cx.notify();`.
- Save.
