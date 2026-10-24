Two more moments come from counting. A **hat-trick** is a player's third goal in one match; the HUD plays a stinger with the player's name. Demolitions (one car blowing up another) are counted per team for the "3 demos" graphic.

**Count per player.** Goals per scorer live in a `HashMap<String, u32>`. `entry(name).or_insert(0)` gives a `&mut u32` to that player's count, starting at 0; `*goals += 1` adds one through it.

**Exactly three.** Fire the hat-trick when the count *becomes* 3, with `== 3`, not `>= 3`. A fourth goal is not a second hat-trick, so it stays quiet.

**The same count on another key.** Counting demolitions per victim has exactly this shape, with a different map and a different number:

```rust
let times = self.victims.entry(demo.victim.name.clone()).or_insert(0);
*times += 1;
if *times == 5 {
    moments.push(Moment::Wrecked(demo.victim.name.clone()));
}
```

**Count per team.** There are only two teams, numbered 0 (blue) and 1 (orange), so `demos` is an array `[u32; 2]`, not a map. Index it with the attacker's `team_num`; an array index is a `usize`, so write `team as usize`. `.min(1)` keeps a strange team number from indexing past the end.

**One arm per event.** `step` is now a `match` on the event, with the `UpdateState` arm from the last quest already written. Add an arm for `GameEvent::GoalScored(goal)` and one for `GameEvent::Demolish(demo)`; `_ => {}` keeps ignoring the rest.

## Your turn

- Add a `GoalScored` arm: count the scorer's goals, and push `Moment::HatTrick(name)` when the count is exactly 3.
- Add a `Demolish` arm: add 1 to the attacker's team in `demos`, and push `Moment::Demo` with that team and its new count. Save.
