To sit left of the tray, the bar needs to know where the taskbar and its tray are. Windows has no "taskbar API" for that. The taskbar is just a window, and you find it the way any app finds a window: by its class name.

**Everything is a window.** To Win32, the taskbar, its clock and your bar are all **windows**: rectangles Windows tracks, each able to hold child windows. Each is made from a **window class**, a named template whose name never changes. The main taskbar's class is `Shell_TrayWnd`; its tray (icons and clock) is a child of class `TrayNotifyWnd`, in every Windows since XP.

**A handle, not the window.** Win32 never hands you the window itself. It gives an **HWND**, a handle: a number that names one window, like a ticket for a coat. You pass it back to other functions ("how big is this one?"). If the window closes, the number goes stale and those calls fail.

**Wide strings.** Win32 takes text as UTF-16, two bytes per character, ending in a zero; Rust's strings are UTF-8 with no zero at the end. `w!("Shell_TrayWnd")` turns a string literal into Win32's form while the program builds, so nothing is converted at run time.

**Look them up.** `FindWindowW(class, None)` finds a top-level window by class. `FindWindowExW(Some(parent), None, class, None)` finds a child of `parent`. Both give a `Result<HWND>`; `.ok()?` gives up with `None` when one is missing. Here it finds the desktop's own window, of class `Progman`:

```rust
let desktop = FindWindowW(w!("Progman"), None).ok()?;
```

**Still unsafe.** These are C functions, like the battery call. A lookup only reads names and gives back a handle, and every handle is checked before use, so one `// SAFETY:` comment covers the block. `window_rect` and `monitor_rect`, written for you, then measure in physical pixels, and `GetDpiForWindow` gives the scaling.

**For the curious.** Windows does have an official "app bar" API, `SHAppBarMessage`, that reserves a strip of the screen for a docked toolbar. Kinetik does not use it: an app bar sits beside the taskbar, taking space from other windows, while the bar wants to sit on it.

## Your turn

- In `find_taskbar`, inside the `unsafe` block, look up the window of class `Shell_TrayWnd`, giving up with `None` if there is none.
- Then look up its child of class `TrayNotifyWnd` the same way.
- Save.
