Alongside the taskbar bar, Kinetik Stats can show a floating panel on the desktop: the widget. The bar's right-click menu now has a "Desktop widget" row that sends a new action, `ToggleWidget`. The same row has to show the widget and hide it again.

**A different window.** The widget is a normal window with no title bar and a see-through background, so only its rounded panel shows. It never takes focus when it opens, so it does not pull the keyboard away from what you are typing. `place_widget` decides where it goes; the window code is already in `toggle_widget_window`.

**Toggle, not open.** The Settings window came to the front when you asked for it twice. The widget behaves differently: asked a second time, it closes. So `toggle_widget_window` first checks whether a widget is already open, closes it and returns. Only if not does it go on to open a new one.

**Take the handle.** `slot` holds the open widget's `WindowHandle`, or `None`. `slot.take()` gives you what was there and leaves `None` behind. Whatever happens next, the slot no longer points at a window that is closing or already gone.

**Through the handle.** A handle cannot touch its window directly; `handle.update(cx, |view, window, cx| ..)` runs a closure inside it. The closure gets three inputs: the window's view, the `window` itself and a `cx`. This quest needs only the window, so the other two are `_`. `update` gives back an error if the window was already closed, and `.is_ok()` turns that into a `bool`.

**The Settings version.** Here is how `open_settings_window` brings an open Settings window to the front. Yours has the same shape, with `take()` instead of looking, and `window.remove_window()`, which closes a window from the inside, instead of `activate_window()`:

```rust
if let Some(handle) = slot
    && handle.update(cx, |_, window, _| window.activate_window()).is_ok()
{
    return;
}
```

**It watches the bar.** `WidgetView` holds the bar's entity and observes it, so every reading the bar takes redraws the widget too.

## Your turn

- At the top of `toggle_widget_window`, take the handle out of `slot`. If removing its window works, return.
- Save. Press Run: the widget opens next to the bar. Right-click the bar and choose Desktop widget to close it.
