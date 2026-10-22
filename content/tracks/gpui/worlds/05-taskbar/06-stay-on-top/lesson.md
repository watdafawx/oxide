The taskbar is itself an always-on-top window. Click it and Windows brings it to the front, over the bar. Kinetik answers by raising its bar again each time. A test cannot click a real taskbar, so the Windows calls go behind a trait first.

**A seam for tests.** `WinApi` lists the few Windows calls the bar needs: the taskbars, the window in front, topmost, move, show. `RealWin` (in `win.rs`) makes the real calls; `FakeWin` records them in `calls`, so a test can check "was the bar raised?".

**The word for it.** A **topmost** window stays above every window that is not topmost too. `SetWindowPos(hwnd, HWND_TOPMOST, ..)` makes it so, with flags that keep its position, size and focus as they are. `RealWin::set_topmost` does that call.

**When to raise.** The `Docker` raises the bar once when it opens. After that, whenever the window in front changes to the taskbar, it raises the bar again:

```rust
if taskbar_in_front { self.api.set_topmost(self.hwnd); }
```

**The handle.** GPUI's window gives its Win32 handle through `raw-window-handle`; `hwnd_of(window)` turns it into an `isize` the Docker keeps.

## Your turn

- In `on_open`, call `self.api.set_topmost(self.hwnd)`.
- In `on_foreground_change`, do the same, only when `taskbar_in_front`.
- Save.
