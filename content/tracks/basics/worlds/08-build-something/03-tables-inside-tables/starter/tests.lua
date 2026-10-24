local t = require("oxide.test")
local teams = require("teams")

t.test("a_win_is_recorded", function()
  local owls = {name = "Owls", wins = 2}
  teams.record_win(owls)
  teams.record_win(owls)
  t.eq(owls.wins, 4, "two more wins")
  t.eq(owls.name, "Owls", "the name stays")
end)

t.test("finds_the_team_with_most_wins", function()
  local list = {
    {name = "Owls", wins = 2},
    {name = "Foxes", wins = 5},
    {name = "Bees", wins = 1},
  }
  t.eq(teams.most_wins(list), "Foxes")
  t.eq(teams.most_wins({{name = "Owls", wins = 3}, {name = "Bees", wins = 3}}), "Owls", "a tie: the first one")
end)
