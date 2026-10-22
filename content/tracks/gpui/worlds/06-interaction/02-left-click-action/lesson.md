Kinetik's General page has a Left-click action: open Task Manager (the default), open Settings, or do nothing. The pill's `on_click` now calls `on_pill_click`, which decides.

**Three choices.** `LeftClick` has three variants, and `ui.left_click` holds the setting. A `match` with one arm each says what happens.

**Starting a program.** Task Manager is `taskmgr`. `ui.launcher` is a `Box<dyn Launcher>`: `RealLauncher` runs the program with `std::process::Command`; tests swap in `FakeLauncher`, which only writes the name down. The same trait-and-fake trick as `WinApi`:

```rust
LeftClick::TaskManager => self.ui.launcher.launch("taskmgr"),
```

**Settings, for now.** The Settings window arrives in the next world. `open_settings` notes the request in `ui.handled`, so the click already goes to the right place.

**Already wired.** The pill's `on_click` from the last quest now calls `on_pill_click(kind, cx)`, so this one method is all a click does.

## Your turn

- In `on_pill_click`, after `self.ui.clicked = Some(kind);`, `match` on `self.ui.left_click`.
- TaskManager launches `"taskmgr"`, Settings calls `self.open_settings(cx)`, Nothing does nothing.
- Save. Press Run and click a pill: Task Manager opens.
