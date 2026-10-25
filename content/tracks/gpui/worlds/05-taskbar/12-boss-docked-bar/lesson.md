Every piece of the docked bar exists: the window options, the taskbar lookup, the dock maths, scaling, topmost, the fullscreen and auto-hide checks, every taskbar. One `tick` a second ties them together. `main.rs` opens a bar per taskbar and, next to each bar, runs a loop that calls its Docker's `tick` and stops when the bar is gone.

**What is given.** Everything in `dock.rs`, `win.rs` and `window.rs`, including `dock_bar`, `bar_visible` and `taskbars_to_use`; the `Docker` struct and `on_open`. `redock` from the last quest is gone: its compare-and-move is the first half of the `tick` you write, and `bar_visible` from the fullscreen quest feeds the second.

**Dock on a change.** Find the primary taskbar; with none, there is nothing to do. Work out the bar's place with `dock_bar`, but keep the `Option` this time instead of using `?`: a hidden taskbar gives `None`, and the hide step below still has to run. When the taskbar differs from `last`, store it; if there is a place, move the bar there and raise it, because a moved window can drop behind the taskbar.

**Show on change.** The bar is visible when the taskbar shows (there is a place) and nothing fullscreen covers its monitor. Call `show` only when that differs from `self.visible`, then remember it: telling Windows "show" every second for a window that is already shown is wasted work, as in `redock`. The shape, with the steps left to you:

```rust
pub fn tick(&mut self) {
    // the primary taskbar, or return
    // its place, kept as an Option
    // changed? remember it; move and raise when there is a place
    // visible? show or hide, only on a change
}
```

**With auto-hide.** With an auto-hidden taskbar, the bar appears when you point at the bottom edge and the taskbar slides up, then hides with it.

**Stopping it.** The bar has no title bar and no close button yet. Stop it from Task Manager, or with `taskkill /IM kinetik_05_boss_docked_bar.exe` in a terminal; the right-click Exit comes in the Interaction world.

## Your turn

- Write the docking half of `tick`: the first three objectives turn green.
- Write the show-or-hide half.
- Press Run: the bar sits on your taskbar.
