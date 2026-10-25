The league table keeps several facts about every team: its name, wins, draws and losses. A table can hold one team's facts together, and a list can hold the teams.

**A record.** `local team = {name = "Owls", wins = 2, draws = 1}` keeps one team's facts together under one name. A map used for the fields of one thing like this is often called a **record**; Java calls it an object and Rust a struct.

**Read a field.** `team.wins` is 2 and `team.name` is "Owls", with the dot you met in world 6. `team.wins = team.wins + 1` records one more win, and since a table is shared by reference, the caller's team changes too.

**A list of records.** Tables can hold tables: `{ {name = "Owls", wins = 2}, {name = "Foxes", wins = 0} }` is a list whose items are records. `teams[1].name` reads "slot 1 of the list, then that team's name".

```lua
local players = {
  {name = "Kim", goals = 3},
  {name = "Lou", goals = 5},
}
for _, player in ipairs(players) do
  print(player.name .. " scored " .. player.goals)
end
```

**Picture it.** Draw it on paper: a column of slots for the list, and in each slot a small card with fields. When an error says `attempt to index a nil value`, you asked for a card that is not there: often a slot past the end, or a misspelled name.

## Your turn

- In `record_win`, add one to `team.wins`. No `return` needed.
- In `most_wins`, walk the list of teams keeping the one with the most wins so far.
- Give back that team's name.
