Taskbars change: the tray grows when an app adds an icon, the taskbar slides up from auto-hide, a monitor is plugged in. Kinetik checks the taskbar every few seconds and moves its bar when something changed. The Docker keeps the last taskbar it docked against and compares.

**Unchanged, do nothing.** `redock` reads the primary taskbar. If it equals `self.last`, nothing moved: give back `None` and do nothing. That is almost every second. The comparison works because `Taskbar` derives `PartialEq`, which compares every field.

**Changed: dock again.** Remember the new taskbar in `self.last`, work out the bar's place with `dock_bar`, and ask `self.api` to move the window there. Only the position changes; the bar's size is GPUI's business. Moving is one more `WinApi` call, the same shape as showing or hiding:

```rust
self.api.show(self.hwnd, false);
```

**Hiding counts too.** An auto-hidden taskbar gives `None` from `dock_bar`, so `?` ends `redock` without moving. It was still stored in `last`, so the next check does nothing until the taskbar changes again. When it slides up, it differs from `last` and the bar follows it.

**Physical pixels.** `move_to` is a Win32 call, so it takes the physical numbers from `dock_bar` as they are. No scaling here.

## Your turn

- In `redock`, after the `last` check, store the taskbar in `self.last`.
- Work out the place with `dock_bar`, ending with `None` when the taskbar is hidden; move the bar to its left and top, and give back the place.
- Save.
