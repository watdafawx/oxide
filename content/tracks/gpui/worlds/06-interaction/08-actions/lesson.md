The same command can come from many places. Settings opens from a left click, the right-click menu, the tray icon, and later a hotkey. If each of those changes the bar directly, there are four copies to keep in step. GPUI's answer is actions.

**The word for it.** An **action** is a named command, a struct with no data. `actions!(kinetik, [OpenSettings, OpenTaskManager, Exit])` declares three. Anything can dispatch one; whoever handles it decides what happens.

**Handle it once.** `.on_action(cx.listener(|this, _: &OpenSettings, _, cx| ..))` on the bar's root runs whenever `OpenSettings` is dispatched in its window. The menu, the tray and the hotkey will all just dispatch:

```rust
.on_action(cx.listener(|this, _: &OpenSettings, _, cx| this.open_settings(cx)))
```

**Focus decides who hears.** A dispatched action starts at the focused element and travels up to the window's root. `bar_layer` already gives the bar `track_focus` with the handle `focus_in` made, so its handlers are on that path.

**Exit quits.** `Exit` notes itself in `ui.handled`, then `cx.quit()` ends the app: Kinetik's right-click Exit.

## Your turn

- In `bar_layer`, add `.on_action` for `OpenSettings` that calls `this.open_settings(cx)`.
- Add one for `OpenTaskManager` that launches `"taskmgr"`, and one for `Exit` that pushes `"exit"` and quits.
- Save.
