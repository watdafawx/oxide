To sit left of the tray, the bar needs to know where the taskbar and its tray are. Windows has no "taskbar API" for that. The taskbar is just a window, and you find it the way any app finds a window: by its class name.

**Names of windows.** The main taskbar's window class is `Shell_TrayWnd`. Inside it, the tray (notification icons and clock) is a child window of class `TrayNotifyWnd`. Every Windows version since XP uses those names.

**Look them up.** `FindWindowW(class, None)` finds a top-level window by class. `FindWindowExW(Some(parent), None, class, None)` finds a child. Both give a `Result<HWND>`; `.ok()?` gives up with `None` when one is missing:

```rust
let taskbar = FindWindowW(w!("Shell_TrayWnd"), None).ok()?;
```

**Then measure.** `window_rect` (written for you, below) asks `GetWindowRect` for a window's rectangle in physical pixels. `monitor_rect` finds the monitor the taskbar is on, and `GetDpiForWindow` gives its scaling.

**Still unsafe.** These are C functions, like the battery call. Window lookups cannot corrupt memory, so one `SAFETY` comment covers the block.

## Your turn

- In `find_taskbar`, inside the `unsafe` block, look up `Shell_TrayWnd` with `FindWindowW`, then its `TrayNotifyWnd` child with `FindWindowExW`, both with `.ok()?`.
- Save.
