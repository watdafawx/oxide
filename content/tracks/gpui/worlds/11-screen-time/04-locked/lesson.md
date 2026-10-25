Press Win+L and walk off with a game open: the game window is still in front and still covers its monitor, so the idle rule keeps counting it. A locked PC needs its own check.

**The secure desktop.** While the lock screen is up, input goes to a separate, secure desktop. A normal program is not allowed to open it, so `OpenInputDesktop` fails. `RealDesk::locked` already treats that failure as "locked".

**Locked beats everything.** Full screen or not, idle or not, a locked PC counts nothing. So the lock goes into the same `if` as the idle rule, joined with `||`, "or": either one is enough to count nothing.

**Stops at true.** `||` looks at its left side first and stops there if it is `true`. Put `desk.locked()` on the left, and a locked PC never even asks for idle time or window sizes. A rule that skips work on battery or during a game has the same shape:

```rust
if on_battery || game_running {
    return;
}
```

**Admin prompts too.** A UAC prompt, the "Do you want to allow this app" box, also uses the secure desktop. For those few seconds Kinetik counts nothing, which is right: you are not using the program behind it.

## Your turn

- In `app_to_count`, give back `None` when `desk.locked()` is true, in the same `if` as `away(..)`.
- Save.
