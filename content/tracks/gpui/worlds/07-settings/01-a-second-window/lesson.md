Until now "open Settings" only noted the request. This world builds the window. The first rule of a settings window: there is only ever one. Click Settings twice and Kinetik brings its open window to the front instead of opening another.

**Keep the handle.** `cx.open_window(..)` gives back a `WindowHandle<SettingsWindow>`, a way to reach that window later. The bar keeps it in `ui.settings_window`, and `open_settings_window` gets that slot to fill. Its options end with `..Default::default()`, the struct update syntax from the hover card: every option not named keeps its default.

**The app opens windows.** `open_settings_window` takes `cx: &mut App`, the context of the whole app, because a window belongs to the app, not to one view. The bar calls it with its own `Context<StatsBar>`, which works anywhere an `&mut App` is wanted.

**Still there, maybe.** A handle outlives its window: close Settings, and the handle remains, pointing at nothing. `handle.update(cx, |view, window, cx| ..)` reaches into the window and runs the closure with its view, the window itself and a context, or gives back an error when the window is gone. To bring it forward, only the middle input matters: `|_, window, _| window.activate_window()`.

**Two checks in one `if`.** `if let .. && ..` checks "there is a handle" and "it still works" in one condition, and the second half runs only if the first matched. The menu could, for example, act only on a point far enough right:

```rust
if let Some(at) = this.ui.menu_at && at.x > px(100.) { /* .. */ }
```

**Front, or new.** If the window is there, `activate_window` brings it to the front and the function is done: `return`. Otherwise it falls through and opens a fresh one, storing the new handle.

## Your turn

- At the top of `open_settings_window`, check `slot`: if it holds a handle and activating its window works, return.
- Save. Press Run: the bar opens with Settings next to it; right-click the bar and choose Settings… again.
