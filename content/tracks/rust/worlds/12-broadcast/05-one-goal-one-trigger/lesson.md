The owner's real request to Psyonix, who make Rocket League: "We get a second `GoalScored` for a single goal (same payload, no extra flag). That double-fires our automation." Until it is fixed, the HUD has to spot the repeat itself, or the goal stinger plays twice.

**What makes a repeat.** Same scorer, same game time, and less than 2 seconds since the first one arrived. A real second goal has a different game time, even from the same player.

**Remember the last.** `GoalDedup` keeps the last goal that fired in `last`: an `Option` of scorer, game time, and when it arrived. It is `None` until the first goal.

**Chained checks.** `if let` and plain conditions chain with `&&` (a **let-chain**): the names unpacked by the `if let` can be used by the conditions after it, and the body runs only when all of them hold. The same idea on a roster:

```rust
if let Some(player) = roster.get(0) && player.team == "Herons" && !player.photo.is_empty() {
    println!("{} leads with a photo", player.nickname);
}
```

**Unpacking a reference.** `if let Some((scorer, time, at)) = &self.last` borrows the tuple's parts instead of moving them out, so `scorer`, `time` and `at` are references. Put `*` in front to compare the values: `*time == goal.goal_time`.

**The feed's clock.** `now` is time since the feed started, not the wall clock. Replays and tests pass their own `now`, so they behave the same on every run. `now.saturating_sub(*at)` is how long ago the last goal came, and it stops at zero instead of going below.

## Your turn

- In `is_repeat`, give back `true` when the last goal has the same scorer and game time, and came less than `REPEAT_WINDOW` ago.
- Otherwise remember this goal in `self.last` and give back `false`. Save.
