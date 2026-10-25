With three monitors, each has its own taskbar. Kinetik's General page has "Show on all taskbars": one bar per taskbar. `RealWin::taskbars` already finds them: the primary `Shell_TrayWnd` first, then every window of class `Shell_SecondaryTrayWnd`.

**Secondary taskbars.** They have no tray, so `RealWin` treats their right end as the tray: the bar sits against the right edge of each secondary taskbar.

**Pick the taskbars.** With the setting off, only the primary taskbar gets a bar; with it on, all of them. A `filter` keeps a taskbar when either is true. It hands its closure a reference, and gives back references too, so `copied()` turns each `&Taskbar` into a `Taskbar` of its own before `collect`. That works because `Taskbar` is `Copy`. The same chain, keeping adapters that are up or all of them:

```rust
adapters.iter().filter(|a| show_all || a.up).copied().collect()
```

**Right monitor, please.** When this track was tested, GPUI turned out to check that a new window lands on its target display. A window meant for monitor 2 falls back to a default-sized window on the primary unless its `WindowOptions` set `display_id`, the display it belongs on. So the boss's `main.rs` keeps `all_taskbars` off; a bar on a second taskbar needs its display's id as well.

**Shown with one setting.** The boss reads "Show on all taskbars" as a plain `bool`; the Settings world gives it a toggle and saves it.

## Your turn

- In `taskbars_to_use`, keep every taskbar when `all_taskbars` is on, else only the primary.
- Save.
