A strip of numbers over a fullscreen video or a game is in the way. Kinetik hides itself while something fullscreen is in front, per monitor: a game on your second screen leaves the bar on the first alone.

**What fullscreen means.** Fullscreen and borderless games both make a window at least as big as their monitor. "Covers" means every edge is at or past the monitor's edge. A maximised window does not count: it stops at the taskbar.

**Which monitor.** `WinApi::foreground` gives the window in front and its monitor. Only a window covering the bar's own monitor hides the bar:

```rust
Some((window, monitor)) => !(monitor == bar_monitor && covers_monitor(window, monitor)),
```

**Nothing in front.** With no foreground window at all, for example on the lock screen, `None` means show. The bar is cheap to show and confusing to lose.

**Pure again.** Two small functions on rectangles. The Docker calls them once a second in the boss.

## Your turn

- In `covers_monitor`, check all four edges.
- In `bar_visible`, hide only when the window in front covers `bar_monitor`.
- Save.
