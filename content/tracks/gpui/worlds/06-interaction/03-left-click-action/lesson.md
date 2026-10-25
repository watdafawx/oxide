Kinetik's General page has a Left-click action: open Task Manager (the default), open Settings, or do nothing. The pill's `on_click` now calls `on_pill_click`, which decides.

**Three choices.** `LeftClick` has three variants, and `ui.left_click` holds the setting. A `match` with one arm each says what happens; an arm that does nothing is `{}`, an empty block.

**Starting a program.** `ui.launcher` is a `Box<dyn Launcher>`: `RealLauncher` runs the program with `std::process::Command`; tests swap in `FakeLauncher`, which only writes the name down. It is the same trait-and-fake trick as `WinApi` in Taskbar. Task Manager's program name is `taskmgr`; opening Notepad would be:

```rust
self.ui.launcher.launch("notepad");
```

**A shared list.** `FakeLauncher` keeps its names in an `Rc<RefCell<Vec<String>>>`. The test keeps one handle to that list and the bar's launcher another, so the test can read what the launcher wrote. `Rc` shares one value between owners and `RefCell` lets either change it; both come from [Rust world 06](/quest/rust/06-smart-pointers/11-shared-and-changeable). Here you only use them, through the starter.

**Settings, for now.** The Settings window arrives in the next world. `open_settings` notes the request in `ui.handled`, so the click already goes to the right place.

**Already wired.** The pill's `on_click` from the last quest now calls `on_pill_click(kind, cx)`, so this one method is all a click does.

## Your turn

- In `on_pill_click`, after `self.ui.clicked = Some(kind);`, `match` on `self.ui.left_click`.
- `TaskManager` launches `"taskmgr"`, `Settings` calls `self.open_settings(cx)`, `Nothing` does nothing.
- Save. Press Run and click a pill: Task Manager opens.
