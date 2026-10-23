The feed never says "hat-trick" or "30 seconds left". rl-hud works these out itself and fires stinger graphics for them. Each one is a small piece of state and a rule.

**Fire once.** "30 seconds left" must fire as the clock passes 30, not on every update after. A flag does it: armed, fired, disarmed, and armed again only when the clock is back above 30 (a new match).

**Overtime starts.** Fire when `overtime` turns from false to true: compare with what the last update said, then remember the new value.

**Count per player.** Goals per scorer in a `HashMap`; the hat-trick fires when a count becomes exactly 3, so a fourth goal stays quiet:

```rust
let goals = self.goals.entry(goal.scorer.name.clone()).or_insert(0);
```

**Count per team.** Demolitions go to the attacker's team, `demos[0]` or `demos[1]`; every demo fires with its team's new total, for the "3 demos" graphic.

**One step function.** `step` takes one event and gives back the moments it causes. No timers, no sockets: easy to test, and the replay check later depends on it.

## Your turn

- In `MomentTracker::step`, handle `UpdateState`: overtime starting, and 30 seconds left once.
- Handle `GoalScored` (hat-trick on 3) and `Demolish` (count per team). Save.
