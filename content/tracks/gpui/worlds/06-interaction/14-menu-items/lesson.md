The menu opens and closes, but its rows do nothing yet. Each row knows its label and its action; a click should close the menu and send the action on its way to the handler you wrote in Actions.

**Close first.** Set `menu_at` to `None` so the menu disappears on the next frame, whatever the action does.

**The window sends it.** `window.dispatch_action(action, cx)` sends an action through the window, from the focused element up, the same path the tests' `cx.dispatch_action` took. So this listener uses its third input, `window`, for the first time.

**A box for any action.** `dispatch_action` takes a `Box<dyn Action>`: a box that can hold any action, the way `Box<dyn Sensor>` held any sensor. For an action you know, `Box::new(..)` makes the box:

```rust
window.dispatch_action(Box::new(OpenSettings), cx);
```

**One row, many clicks.** `menu_row` gets its action as `action: impl Action`, some action type, and every click must send it again. `action.boxed_clone()` makes a fresh box holding a copy each time, and the row keeps its own. The closure takes `action` with `move`, because it outlives `menu_row`.

**One handler.** The row never opens Settings itself. `OpenSettings` reaches `bar_layer`'s handler, which calls `open_settings`, the same as a left click with Settings chosen, and soon the tray icon.

## Your turn

- In `menu_row`, add `.on_click(cx.listener(move |this, _, window, cx| { .. }))` to the row.
- Inside: close the menu, dispatch `action.boxed_clone()` through the window, notify.
- Save. Press Run, right-click a pill, and choose Exit.
