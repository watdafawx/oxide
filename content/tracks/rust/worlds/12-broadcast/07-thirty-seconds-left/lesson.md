The feed never says "30 seconds left" or "overtime". It only sends the clock, several times a second. rl-hud works these moments out itself and plays a stinger for each, so each one needs a little memory and a rule.

**Changes, not states.** "Overtime" should fire when overtime *starts*, not on every update while it lasts. So compare what this update says with what the last one said, then remember the new value for next time. That is all `self.overtime` is for.

**The same trick on the score.** Here is that compare-then-remember shape for a different moment, blue's score going up:

```rust
let blue = game.teams[0].score;
if blue > self.last_blue {
    moments.push(Moment::BlueScored);
}
self.last_blue = blue;
```

**Fire once.** "30 seconds left" fires as the clock gets to 30 or below, not on every update after. A flag does it: `thirty_armed` starts `true`; firing sets it to `false`, so the next updates stay quiet. A clock above 30 again (a new match) arms it again.

**Not in overtime.** In overtime the clock counts *up* from 0, so it is below 30 for half a minute. Check `!game.overtime` before firing, or the HUD shouts "30 seconds left" at the start of overtime.

**One step function.** `step` takes one event and gives back the moments it causes. No timers, no sockets: easy to test, and the replay check later depends on it.

## Your turn

- In `step`, push `Moment::Overtime` when `game.overtime` turns true, then remember it in `self.overtime`.
- Push `Moment::ThirtySecondsLeft` once as the clock gets to 30 or below outside overtime; a clock above 30 arms it again. Save.
