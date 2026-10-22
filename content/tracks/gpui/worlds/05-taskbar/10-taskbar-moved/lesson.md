Taskbars change: the tray grows when an app adds an icon, the taskbar slides up from auto-hide, a monitor is plugged in. Kinetik checks the taskbar every few seconds and moves its bar when something changed. The Docker keeps the last taskbar it docked against and compares.

**Unchanged, do nothing.** `redock` reads the primary taskbar. If it equals `self.last`, nothing moved: give back `None` and do nothing. That is almost every second.

**Changed: dock again.** Remember the new taskbar, work out the bar's place with `dock_bar`, and move the window there. Only position changes; the bar's size is GPUI's business:

```rust
self.api.move_to(self.hwnd, place.left, place.top);
```

**Hiding counts too.** An auto-hidden taskbar gives `None` from `dock_bar`, so `?` ends `redock` without moving. When it slides up, it differs from `last` again and the bar follows it.

**Physical pixels.** `move_to` is a Win32 call, so it takes the physical numbers from `dock_bar` as they are. No scaling here.

## Your turn

- In `redock`, after the `last` check, store `Some(taskbar)` in `self.last`.
- Get the place with `dock_bar(..)?`, call `move_to`, and give back `Some(place)`.
- Save.
