The menu opens and closes, but its rows do nothing yet. Each row knows its label and its action; a click should close the menu and send the action on its way to the handler you wrote two quests ago.

**Close first.** Set `menu_at` to `None` so the menu disappears on the next frame, whatever the action does.

**Then dispatch.** `window.dispatch_action(..)` sends an action through the window, from the focused element up, the same path `cx.dispatch_action` took in the tests. It wants a `Box<dyn Action>`; `action.boxed_clone()` makes one from the row's action:

```rust
window.dispatch_action(action.boxed_clone(), cx);
```

**One handler.** The row never opens Settings itself. `OpenSettings` reaches `bar_layer`'s handler, which calls `open_settings`, the same as a left click with Settings chosen, and soon the tray icon.

**`move` for the action.** The row's closure outlives `menu_row`, so it takes the action with `move`; `boxed_clone` makes a fresh box each click.

## Your turn

- In `menu_row`, add `.on_click(cx.listener(move |this, _, window, cx| { .. }))` to the row.
- Inside: close the menu, dispatch `action.boxed_clone()`, notify.
- Save. Press Run, right-click a pill, and choose Exit.
