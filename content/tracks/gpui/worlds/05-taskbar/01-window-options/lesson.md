Until now the bar lived in an ordinary window: title bar, close button, its own button on the taskbar. Kinetik's bar is none of that. It is a strip of numbers that sits on the taskbar and never gets in the way. That starts with the window's options.

**Options, field by field.** `WindowOptions` is a plain struct that `cx.open_window` reads once, when it makes the window. `..Default::default()` fills every field you leave out with its default, so you name only what differs from an ordinary window.

**No title bar.** `titlebar: None` removes the title bar and the frame. All that is left is what the view draws.

**A pop-up.** `kind: WindowKind::PopUp` makes a window that floats above normal windows and, on Windows, never gets a taskbar button of its own. GPUI gives it the `WS_EX_TOOLWINDOW` style for that: a Win32 flag for small tool windows, which the taskbar leaves out.

**Hands off.** `is_movable: false` and `is_resizable: false` stop drags and edge resizes. `focus: false` matters most: the window with **focus** gets the keyboard, and opening or clicking the bar must never take it away from the game or editor you are typing in.

**One function.** `bar_window_options(bounds)` gathers all of it, so every bar window, on every taskbar, opens the same way. A settings window, by contrast, wants the ordinary choices, written out:

```rust
kind: WindowKind::Normal, is_movable: true, is_resizable: true, focus: true,
```

## Your turn

- In `bar_window_options`, inside the `WindowOptions { .. }`, set five fields: no title bar, the pop-up kind, not movable, not resizable, no focus.
- Save. Press Run: a bare strip, no frame, no taskbar button. Close it from the terminal or Task Manager.
