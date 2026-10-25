A strip of numbers over a fullscreen video or a game is in the way. Kinetik hides itself while something fullscreen is in front, per monitor: a game on your second screen leaves the bar on the first alone.

**No fullscreen flag.** Windows has no "is this app fullscreen?" question to ask. Apps only make windows, and a fullscreen game or video simply makes one the size of its monitor. So the bar looks at the window in front and measures it.

**What covers means.** Fullscreen and borderless games both make a window at least as big as their monitor. "Covers" means every edge is at or past the monitor's: left and top at or before it, right and bottom at or after it. A maximised window does not count: it stops at the taskbar. The opposite check, a window fully inside its monitor, looks like this:

```rust
window.left >= monitor.left && window.right <= monitor.right
```

**Which monitor.** `WinApi::foreground` gives the window in front and its monitor, as `Option<(Rect, Rect)>`. Only a window that covers the bar's own monitor hides the bar; a match arm `Some((window, monitor)) => ..` takes the pair apart.

**Nothing in front.** With no foreground window at all, for example on the lock screen, `None` means show. The bar is cheap to show and confusing to lose.

**Pure again.** Two small functions on rectangles. The Docker calls them once a second in the boss.

## Your turn

- In `covers_monitor`, check all four edges.
- In `bar_visible`, hide only when the window in front covers `bar_monitor`; show when there is none.
- Save.
