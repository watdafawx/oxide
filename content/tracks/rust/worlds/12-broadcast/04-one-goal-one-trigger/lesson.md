The owner's real request to Psyonix: "We get a second `GoalScored` for a single goal (same payload, no extra flag). That double-fires our automation." Until it is fixed, the HUD has to spot the repeat itself.

**What makes a repeat.** Same scorer, same game time, and less than 2 seconds since the first. A real second goal has a different game time, even from the same player.

**Remember the last.** `GoalDedup` keeps the last goal that fired: scorer, game time, and when it arrived on the feed's clock.

**Chained checks.** `if let` and plain conditions chain with `&&`, so one `if` unpacks the last goal and compares all three:

```rust
if let Some((scorer, time, at)) = &self.last && *scorer == goal.scorer.name
```

**The feed's clock.** `now` is time since the feed started, not the wall clock. Replays and tests pass their own `now`, so they behave the same every run.

## Your turn

- In `is_repeat`, give back `true` when the last goal matches scorer and time within `REPEAT_WINDOW`.
- Otherwise remember this goal and give back `false`. Save.
