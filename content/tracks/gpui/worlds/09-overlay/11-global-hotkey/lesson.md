Kinetik toggles its overlay with Ctrl+Shift+F10, "which also works from inside a game". GPUI's key bindings from the Settings world only hear keys sent to a focused GPUI window. In a game, the game has focus.

**Heard everywhere.** A global hotkey is registered with Windows itself (`RegisterHotKey`): whatever has focus, Windows hands that key combination to you. The `global-hotkey` crate wraps it; its events arrive fine while GPUI runs its own loop.

**Describe the keys.** `HotKey::new(modifiers, key)` names the combination. The modifiers are an `Option`, `None` for a bare key. Several modifiers held together are one value, joined with `|`, the same "set these bits" `|` as the widget's click-through. Here is Alt+Shift+B:

```rust
HotKey::new(Some(Modifiers::ALT | Modifiers::SHIFT), Code::KeyB)
```

**Press, not release.** One keystroke sends two events, `HotKeyState::Pressed` and `HotKeyState::Released`; log them and you see both. Acting on both would show and hide the overlay in one keystroke. So `hotkey_action` returns `None` at once for anything that is not a press.

**Same action.** A Pressed event becomes `ToggleOverlay`, the same action the bar's menu row dispatches.

## Your turn

- In `overlay_hotkey`, give back Ctrl+Shift+F10.
- At the top of `hotkey_action`, return `None` unless `event.state` is `HotKeyState::Pressed`.
- Save.
