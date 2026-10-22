Kinetik's Stats page has a box for the ping address. GPUI 0.2.2 has no text box widget, and a full one (selection, cursor, IME) is a project of its own. Kinetik needs an address, so this box keeps it simple: type at the end, Backspace removes the last character.

**Keys go to focus.** Key presses go to the focused element. The box tracks its own `address_focus`; clicking it calls `window.focus(..)`, so the next keys are its.

**Two kinds of key.** A `KeyDownEvent` has a `keystroke`. `key` names the key (`"backspace"`, `"a"`, `"."`); `key_char` is the text it types, or `None` for keys like Shift that type nothing:

```rust
if key.key == "backspace" { s.ping_address.pop(); } else if let Some(typed) = &key.key_char { s.ping_address.push_str(typed); }
```

**Straight to Settings.** Each key updates `ping_address` in the global, so the address is always saved state, never a half-typed copy.

**Used on restart.** The ping sensor reads the address when the bar starts. Changing it live would mean rebuilding a sensor on the background thread, which this track does not do.

## Your turn

- In `address_box`, add `.on_key_down(cx.listener(|_, event: &KeyDownEvent, _, cx| { .. }))`.
- Inside, update the global: Backspace pops, any `key_char` is pushed; then notify.
- Save. Press Run, click the box on the Stats page and type.
