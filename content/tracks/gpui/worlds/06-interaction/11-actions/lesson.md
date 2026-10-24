The same command can come from many places. Settings opens from a left click, the right-click menu, the tray icon, and later a hotkey. If each of those changes the bar directly, there are four copies to keep in step. GPUI's answer is actions.

**The word for it.** An **action** is a named command, a struct with no data. `actions!(kinetik, [OpenSettings, OpenTaskManager, Exit])` at the top of `lib.rs` declares three. Anything can dispatch one; whoever handles it decides what happens.

**The type picks the handler.** `.on_action(cx.listener(..))` takes the usual four-input closure, and its event input is the action. Writing its type, `_: &OpenSettings`, is what tells GPUI which action this handler is for. The value itself carries nothing, so it is named `_`. The starter's handler for Task Manager:

```rust
.on_action(cx.listener(|this, _: &OpenTaskManager, _, _| this.ui.launcher.launch("taskmgr")))
```

**Focus decides who hears.** An action starts at the focused element and travels up, as the last quest showed. The bar's root tracks the bar's focus handle, so its handlers are on that path.

**Exit quits.** `Exit` notes itself in `ui.handled`, then `cx.quit()` ends the app: Kinetik's right-click Exit.

## Your turn

- In `bar_layer`, below the `OpenTaskManager` handler, add one for `OpenSettings` that calls `this.open_settings(cx)`.
- Add one for `Exit` that pushes `"exit"` onto `this.ui.handled`, then calls `cx.quit()`.
- Save.
