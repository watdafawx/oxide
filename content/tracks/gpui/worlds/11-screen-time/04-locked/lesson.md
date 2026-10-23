Press Win+L and walk off with a game open: the game window is still in front and still covers its monitor, so the idle rule would keep counting it. A locked PC needs its own check.

**The secure desktop.** While the lock screen is up, input goes to a separate, secure desktop. A normal program cannot open it, so `OpenInputDesktop` fails. `RealDesk::locked` treats that failure as "locked".

**Check it first.** Locked beats everything, full screen included. One `||` in front of the idle check:

```rust
if desk.locked() || away(desk.idle(), desk.fullscreen()) {
```

**Stops at true.** `||` stops at the first `true`, so a locked PC never asks for idle time or window sizes.

**Admin prompts too.** A UAC prompt also uses the secure desktop. For those few seconds Kinetik counts nothing, which is right: you are not using the program behind it.

## Your turn

- In `app_to_count`, give back `None` when `desk.locked()`, in the same `if` as `away`.
- Save.
