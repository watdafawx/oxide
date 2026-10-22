Alongside the taskbar bar, Kinetik Stats can show a floating panel on the desktop: the widget. Its right-click menu has "Desktop widget" to turn it on and off. `ToggleWidget` is a new action, and the bar's menu now has a row for it.

**A different window.** The widget is a normal window with no title bar and a see-through background, so only its rounded panel shows. It is never focused when it opens. `place_widget` decides where it goes.

**Toggle, not open.** The Settings window came to the front when asked twice. The widget closes: the same menu row both shows and hides it. `slot.take()` hands you the handle and leaves `None`, so a closed widget is forgotten either way.

**Close it.** `window.remove_window()` closes a window from the inside. Called through the handle, it also tells you whether the window was still there:

```rust
if let Some(handle) = slot.take() && handle.update(cx, |_, window, _| window.remove_window()).is_ok() { return; }
```

**It watches the bar.** `WidgetView` holds the bar's entity and observes it, so every reading the bar takes redraws the widget too.

## Your turn

- At the top of `toggle_widget_window`, take the handle out of `slot`; if removing its window works, return.
- Save. Press Run: the widget opens next to the bar; right-click the bar and choose Desktop widget.
