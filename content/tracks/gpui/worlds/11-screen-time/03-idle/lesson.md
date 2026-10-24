Leave the PC on and walk away, and the window in front would keep collecting time. Kinetik stops counting after 5 minutes without input, the same rule Windows uses to dim the screen.

**Last input.** `GetLastInputInfo` gives the moment of the last key press or mouse move, as a tick count. Now minus that is the idle time. `RealDesk::idle` already does it; ticks wrap back to 0 after 49 days, so it subtracts with `wrapping_sub`.

**Watching is using.** A film or a game cutscene runs for minutes while nobody touches anything. If the window in front covers its whole monitor, idle time does not matter. `RealDesk::fullscreen` reuses `covers_monitor` from the taskbar world.

**One rule, one function.** `away` holds the rule on its own, so it can be tested without a desk: away means idle for at least `IDLE_LIMIT`, and the window in front is not full screen.

**Durations compare.** Two `Duration`s compare with `>=` like numbers. `&&` joins two conditions and `!` turns `true` into `false`, so a rule with a "not" reads almost like the sentence. A download that gave up waiting:

```rust
let gave_up = waited >= Duration::from_secs(30) && !cancelled;
```

**A constant length.** `Duration::from_secs(5 * 60)` can be worked out before the program runs, so `IDLE_LIMIT` is a `const`, like `SAMPLE_SECS`.

## Your turn

- Write `away`: `true` when `idle` is at least `IDLE_LIMIT` and the window is not full screen.
- Save. `app_to_count` already asks `away` first.
