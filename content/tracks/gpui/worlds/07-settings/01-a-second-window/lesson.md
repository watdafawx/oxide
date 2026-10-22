Until now "open Settings" only noted the request. This world builds the window. The first rule of a settings window: there is only ever one. Click Settings twice and Kinetik brings its open window to the front instead of opening another.

**Keep the handle.** `cx.open_window(..)` gives back a `WindowHandle<SettingsWindow>`. The bar keeps it in `ui.settings_window`, and `open_settings_window` gets that slot to fill.

**Still there, maybe.** A handle outlives its window: close Settings, and the handle remains, pointing at nothing. `handle.update(cx, ..)` tries to reach the window and gives back an error when it is gone. `if let .. &&` checks "there is a handle" and "it still works" in one condition:

```rust
if let Some(handle) = slot && handle.update(cx, |_, window, _| window.activate_window()).is_ok() { return; }
```

**Front, or new.** If the window is there, `activate_window` brings it to the front and the function is done. Otherwise it falls through and opens a fresh one, storing the new handle.

**Not focus-free.** The bar never takes focus, but Settings is a normal window: you type into it. Its options have a title bar and the defaults.

## Your turn

- In `open_settings_window`, before opening, check `slot`: if it holds a handle and activating it works, return.
- Save. Press Run: the bar opens with Settings next to it; right-click the bar and choose Settings… again.
