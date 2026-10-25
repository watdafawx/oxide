A new CPU reading arrives. Changing the snapshot and the items is not enough: GPUI does not watch your fields, and it draws only when something tells it to. Without that signal, the numbers change in memory and the screen keeps showing the old ones.

**Change through the app.** `bar.update(cx, |bar, cx| ..)` lends a `&mut StatsBar` and a `&mut Context<StatsBar>` for the length of the closure. The context is the app, seen from this one entity: it is how the bar talks back to GPUI while it changes.

**Then say so.** `cx.notify()` on that context tells GPUI "this entity changed": its window draws again on the next frame, and anything watching the bar hears about it.

**Forget it once.** Run the starter's test before you change anything. The text is right, but the test counted no notify. That is the classic GPUI bug: the state is new and the screen is stale.

```text
assertion `left == right` failed: set_reading should call cx.notify() once
  left: 0
 right: 1
```

**Notify, don't draw.** You never call `render` yourself. Notify is cheap; GPUI collects all the notifies of one frame and draws once.

## Your turn

- At the end of `set_reading`, tell GPUI the bar changed.
- Save. The count is now 1.
