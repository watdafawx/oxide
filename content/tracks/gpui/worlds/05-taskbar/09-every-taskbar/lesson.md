With three monitors, each has its own taskbar. Kinetik's General page has "Show on all taskbars": one bar per taskbar. `RealWin::taskbars` already finds them: the primary `Shell_TrayWnd` first, then every `Shell_SecondaryTrayWnd`.

**Secondary taskbars.** They have no tray, so `RealWin` treats their right end as the tray: the bar sits against the right edge of each secondary taskbar.

**Pick the taskbars.** With the setting off, only the primary taskbar gets a bar; with it on, all of them. A `filter` keeps a taskbar when either is true, and `copied()` turns the borrowed taskbars into owned ones:

```rust
taskbars.iter().filter(|t| all_taskbars || t.primary).copied().collect()
```

**Right monitor, please.** Testing found a trap: GPUI checks that a new window lands on its target display, and a window meant for monitor 2 without `display_id` set falls back to a default-sized window on the primary. The boss sets `display_id` for each bar.

**Shown with one setting.** The boss reads "Show on all taskbars" as a plain `bool` for now; the Settings world gives it a toggle and saves it.

## Your turn

- In `taskbars_to_use`, keep every taskbar when `all_taskbars` is on, else only the primary.
- Save.
