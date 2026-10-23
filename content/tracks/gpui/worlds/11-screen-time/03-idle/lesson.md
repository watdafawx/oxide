Leave the PC on and walk away, and the window in front would keep collecting time. Kinetik stops counting after 5 minutes without input, the same rule Windows uses to dim the screen.

**Last input.** `GetLastInputInfo` gives the tick count of the last key press or mouse move. Now minus that is the idle time. `RealDesk::idle` does it; ticks wrap after 49 days, so it uses `wrapping_sub`.

**Watching is using.** A film or a game cutscene runs for minutes with nobody touching anything. If the window in front covers its whole monitor, idle does not matter. `RealDesk::fullscreen` reuses `covers_monitor` from the taskbar world.

**One rule, one function.** `away` holds the rule, so it can be tested without a desk:

```rust
idle >= IDLE_LIMIT && !fullscreen
```

**Duration compares.** `Duration` values compare with `>=` like numbers; `Duration::from_secs(5 * 60)` is a constant, so `IDLE_LIMIT` can be a `const`.

## Your turn

- Write `away`: idle at least `IDLE_LIMIT`, and not full screen.
- Save. `app_to_count` already asks `away` first.
