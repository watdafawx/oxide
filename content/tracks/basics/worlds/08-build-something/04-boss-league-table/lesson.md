Games night: four friends play a tournament and want a live league table on the screen. You have every piece you need. The boss is about putting them together, one small, tested function at a time.

**The plan.** A team is a record with four fields, and the league is a list of teams. Six small functions do all the work, and each stage of the objectives tests one of them, so the list turns green from the top down as you go.

**Stages 1 and 2: teams.** `new_team(name)` makes a record whose name is `name` and whose counts are all 0. `find(teams, name)` walks the list and gives back the team with that name, or nil when there is none: the stop-early search from world 4.

```lua
-- a team record, and the shape of a call
local owls = {name = "Owls", wins = 2, draws = 1, losses = 1}
M.record_result(league, "Owls", "Bees", 3, 1)   -- Owls 3, Bees 1: a win for the Owls
```

**Stage 3: numbers.** `points(team)` is 3 for each win and 1 for each draw; `played(team)` is every game the team has played. These are the debugged functions from "Narrow it down", now reading a record's fields.

**Stage 4: a result.** `record_result` finds the home team and the away team, then decides what happened: more home goals is a home win and an away loss; more away goals is the opposite; equal goals is a draw for both. Because tables are shared by reference, changing the found team changes the team in the list.

**Stages 5 and 6: the table.** `leader(teams)` is the champion pattern over points, keeping the earlier team on a tie. `table_lines(teams)` builds a list of text lines, one per team, in the list's order. When everything is green, `src/main.lua` prints a whole tournament with your code.

## Your turn

- In `src/league.lua`, follow the `TODO (stage N)` comments from stage 1 upwards.
- Save after each stage and watch the next objective turn green.
- When all six are green, press Run to see the final table of a whole tournament.
