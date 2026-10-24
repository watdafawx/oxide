Kinetik toggles its overlay with Ctrl+Shift+F10, "which also works from inside a game". GPUI's key bindings from the Settings world only hear keys sent to a focused GPUI window. In a game, the game has focus.

**Heard everywhere.** A global hotkey is registered with Windows itself (`RegisterHotKey`): whatever has focus, Windows hands that key combination to you. The `global-hotkey` crate wraps it; its events arrive fine while GPUI runs its own loop.

**Describe the keys.** `HotKey::new(modifiers, key)` names the combination. Control and Shift together are one value, joined with `|`:

```rust
HotKey::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::F10)
```

**Press, not release.** One press sends two events, Pressed and Released (you will see both if you log them). Acting on both would show and hide the overlay in one keystroke. Act on Pressed only.

**Same action.** A Pressed event becomes `ToggleOverlay`, the same action the bar's menu row dispatches.

## Your turn

- In `overlay_hotkey`, give back Ctrl+Shift+F10.
- At the top of `hotkey_action`, return `None` unless `event.state` is `HotKeyState::Pressed`.
- Save.
