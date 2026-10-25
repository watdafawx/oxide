Kinetik's Stats page has a box for the ping address. GPUI 0.2.2 has no text box widget, and a full one (selection, cursor, input methods for other scripts) is a project of its own. Kinetik needs an address, so this box keeps it simple: type at the end, Backspace removes the last character.

**Keys go to focus.** Key presses go to the focused element, as actions did in Interaction. The box tracks its own focus handle, `address_focus`; clicking it calls `window.focus(..)`, so the next keys are its.

**Two kinds of key.** `on_key_down`'s listener gets a `KeyDownEvent`, whose `keystroke` describes the key. `keystroke.key` names it (`"backspace"`, `"a"`, `"."`); `keystroke.key_char` is the text it types, an `Option<String>` that is `None` for keys like Shift that type nothing. A box that only reported what was typed would read:

```rust
if let Some(typed) = &event.keystroke.key_char { println!("typed {typed}"); }
```

**Backspace first.** Backspace types no text, so check `key == "backspace"` first and `pop()` the last character; otherwise, push whatever `key_char` holds with `push_str`.

**Straight to Settings.** Each key updates `ping_address` in the global, through `Settings::update_global`, so the address is always saved state, never a half-typed copy.

**Used on restart.** The ping sensor reads the address when the bar starts. Changing it live would mean rebuilding a sensor on the background thread, which this track does not do.

## Your turn

- In `address_box`, add `.on_key_down(cx.listener(|_, event: &KeyDownEvent, _, cx| { .. }))`.
- Inside, update the global: Backspace pops, any `key_char` is pushed; then notify.
- Save. Press Run, click the box on the Stats page and type.
