Kinetik's Arrange page has keys: select a row, press Alt+Up or Alt+Down to move it. In GPUI, keys become actions through key bindings. You wrote `move_up` in Readings and actions in Interaction; this joins them.

**Keys to actions.** `KeyBinding::new("alt-up", MoveUp, ..)` says "Alt+Up dispatches `MoveUp`". `cx.bind_keys([..])` registers bindings for the whole app, once, at startup.

**Only here.** The third argument is a key context. `Some("ArrangeList")` makes the binding apply only while focus is inside an element marked `.key_context("ArrangeList")`, which the Settings window is. Alt+Up anywhere else does its usual thing:

```rust
KeyBinding::new("alt-up", MoveUp, Some("ArrangeList")),
```

**Handled already.** The window's `on_action` handlers call `move_selected`, which uses your `move_up` and moves the selection with the row, so a second Alt+Up keeps going.

**Click selects and focuses.** Clicking a row selects it and focuses the list, so the keys reach it.

## Your turn

- In `bind_settings_keys`, call `cx.bind_keys([..])` with Alt+Up for `MoveUp` and Alt+Down for `MoveDown`, both in `"ArrangeList"`.
- Save. Press Run, open Arrange, click a row and press Alt+Up.
