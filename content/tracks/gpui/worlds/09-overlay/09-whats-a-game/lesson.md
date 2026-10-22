"Show automatically in games" needs a rule for "a game is in front". Kinetik's: a window that fills its screen, fullscreen or borderless. Windowed games do not count, and Kinetik's README says so plainly.

**Reuse the taskbar rule.** You wrote `covers_monitor` to hide the bar in fullscreen. A game is the same shape: the window in front covers its monitor.

**Not the desktop.** The desktop itself is a window of `explorer.exe` that covers the screen, and so is the lock screen. `NOT_GAMES` lists those; any of them is not a game.

**Case does not matter.** Windows file names ignore case: `Explorer.EXE` is the same program. `eq_ignore_ascii_case` compares that way:

```rust
!NOT_GAMES.iter().any(|name| exe.eq_ignore_ascii_case(name))
```

**Borderless, please.** Exclusive fullscreen games also cover their monitor, but no overlay can draw over them. Kinetik's tip: switch the game to borderless.

## Your turn

- In `is_game`, give back true when the window covers its monitor and the exe is not in `NOT_GAMES`.
- Save.
