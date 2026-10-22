Until now the bar lived in an ordinary window: title bar, close button, its own button on the taskbar. Kinetik's bar is none of that. It is a strip of numbers that sits on the taskbar and never gets in the way. That starts with the window's options.

**No title bar.** `titlebar: None` removes the title bar and the frame. All that is left is what the view draws.

**A pop-up.** `kind: WindowKind::PopUp` makes a window that floats above normal windows and, on Windows, never gets a taskbar button of its own. GPUI gives it the `WS_EX_TOOLWINDOW` style for that.

**Hands off.** `is_movable: false` and `is_resizable: false` stop drags and edge resizes. `focus: false` matters most: opening or clicking the bar must never pull the keyboard away from the game or editor you are using.

**One function.** `bar_window_options(bounds)` gathers all of it, so every bar window, on every taskbar, opens the same way:

```rust
titlebar: None, kind: WindowKind::PopUp, is_movable: false, is_resizable: false, focus: false,
```

## Your turn

- In `bar_window_options`, set the five fields above inside the `WindowOptions { .. }`.
- Save. Press Run: a bare strip, no frame, no taskbar button. Close it from the terminal or Task Manager.
