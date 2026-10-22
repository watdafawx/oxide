The Settings window is complete: pages, toggles, the gap slider, swatches, the address box, the Arrange list with drag and keys, saving and defensive loading. Three things are left to make it Kinetik's: the bar following every setting, settings that survive a restart, and profiles.

**The bar follows.** `apply_settings` copies what the bar shows from a `Settings`: a `Look` from its five look fields, the order, the colours, the left-click action, and fresh items from the new order. `watch_settings` calls it on every change, so this one method makes the window live.

**Saved on change.** `persist_settings(path, cx)` watches the global too, at app level: `App::observe_global` takes a closure that gets only `cx`. Every change writes the file with your `save`. `main.rs` keeps the subscription with `.detach()`.

**Swap whole setups.** Kinetik's right-click menu switches between saved setups like Gaming and Work. `Profiles` keeps whole `Settings` by name; switching sets the global, and everything watching it follows:

```rust
cx.set_global(settings.clone());
```

**Already right.** `load` from the last quest makes a broken file harmless, so the last objective passes as soon as everything compiles.

## Your turn

- Write `apply_settings`: five objectives depend on it.
- Write `persist_settings`.
- Write `Profiles::save_current` and `Profiles::switch_to`.
- Press Run: change things, close, run again.
