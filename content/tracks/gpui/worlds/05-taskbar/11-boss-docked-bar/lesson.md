Every piece of the docked bar exists: the window options, the taskbar lookup, the dock maths, scaling, topmost, fullscreen and auto-hide checks, every taskbar. One `tick` a second ties them together. `main.rs` opens a bar per taskbar and runs a Docker's `tick` in a loop next to it.

**What is given.** Everything in `dock.rs`, `win.rs` and `window.rs`, the `Docker` struct and `on_open`. The world's earlier `redock` and fullscreen check are folded into the one `tick` you write.

**Dock on a change.** Find the primary taskbar and work out the bar's place with `dock_bar`, keeping the `Option`. When the taskbar differs from `last`, store it; if there is a place, move the bar and raise it, because a moved window can drop behind the taskbar.

**Show on change.** The bar is visible when the taskbar shows (there is a place) and nothing fullscreen covers its monitor. Call `show` only when that differs from `self.visible`:

```rust
let visible = docked.is_some() && bar_visible(self.api.foreground(), taskbar.monitor);
```

**On this PC.** With an auto-hidden taskbar, the bar appears when you point at the bottom edge and the taskbar slides up, then hides with it.

## Your turn

- Write the docking half of `tick`: the first three objectives turn green.
- Write the show-or-hide half.
- Press Run: the bar sits on your taskbar. Stop it from Task Manager (`kinetik_05_boss_docked_bar.exe`); the right-click Exit comes in the Interaction world.
