The taskbar is itself an always-on-top window. Click it and Windows brings it to the front, over the bar. Kinetik answers by raising its bar again each time. The `WinApi` seam from the last quest is now in `dock.rs`, so the tests can check every raise on a `FakeWin`.

**Windows in a stack.** Windows keeps every window in one front-to-back order, the **z-order** (z is the axis that points out of the screen). Clicking a window moves it to the front of the stack; whatever is in front covers whatever is behind where they overlap.

**The word for it.** A **topmost** window sits in a separate band at the front of that stack, above every window that is not topmost. The taskbar is topmost too, so among the two, the one raised last wins. `RealWin::set_topmost` calls `SetWindowPos(hwnd, HWND_TOPMOST, ..)` with flags that keep the bar's position, size and focus as they are.

**The bar's handle.** Win32 calls need the bar's HWND. GPUI's window gives it through the `raw-window-handle` crate, a common way for windowing libraries to hand out the platform's own handle; `hwnd_of(window)` turns it into an `isize` that the `Docker` keeps in `self.hwnd`.

**When to raise.** The `Docker` raises the bar once when it opens, in `on_open`. After that, whenever the window in front changes, `on_foreground_change` is told whether the taskbar is now in front, and raises the bar only then. Every request goes through `self.api`, never to Windows directly. The same shape, showing the bar again when the taskbar returns:

```rust
if taskbar_back { self.api.show(self.hwnd, true); }
```

## Your turn

- In `on_open`, ask `self.api` to make `self.hwnd` topmost.
- In `on_foreground_change`, do the same, only when the taskbar is in front.
- Save.
