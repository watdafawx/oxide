local t = require("oxide.test")
local league = require("league")

-- a small league for the later stages: Owls 2-1-1, Foxes 1-0-1, Bees 0-0-0
local function sample()
  return {
    {name = "Owls", wins = 2, draws = 1, losses = 1},
    {name = "Foxes", wins = 1, draws = 0, losses = 1},
    {name = "Bees", wins = 0, draws = 0, losses = 0},
  }
end

t.test("stage1_creates_a_team", function()
  t.deep_eq(league.new_team("Owls"), {name = "Owls", wins = 0, draws = 0, losses = 0})
  t.eq(league.new_team("Bees").name, "Bees")
end)

t.test("stage2_finds_a_team_by_name", function()
  local teams = sample()
  t.eq(league.find(teams, "Foxes"), teams[2], "the Foxes record from the list itself")
  t.eq(league.find(teams, "Bees"), teams[3])
  t.is_nil(league.find(teams, "Moles"), "no team called Moles")
end)

t.test("stage3_counts_points_and_games", function()
  local teams = sample()
  t.eq(league.points(teams[1]), 7, "2 wins and 1 draw")
  t.eq(league.points(teams[3]), 0)
  t.eq(league.played(teams[1]), 4, "2 + 1 + 1 games")
  t.eq(league.played(teams[2]), 2, "1 win and 1 loss")
end)

t.test("stage4_records_a_result", function()
  local teams = {
    {name = "Owls", wins = 0, draws = 0, losses = 0},
    {name = "Bees", wins = 0, draws = 0, losses = 0},
  }
  league.record_result(teams, "Owls", "Bees", 3, 1)
  t.deep_eq(teams[1], {name = "Owls", wins = 1, draws = 0, losses = 0}, "a home win")
  t.deep_eq(teams[2], {name = "Bees", wins = 0, draws = 0, losses = 1}, "an away loss")
  league.record_result(teams, "Owls", "Bees", 0, 2)
  t.eq(teams[2].wins, 1, "an away win")
  t.eq(teams[1].losses, 1, "a home loss")
  league.record_result(teams, "Bees", "Owls", 1, 1)
  t.eq(teams[1].draws, 1, "a draw for the Owls")
  t.eq(teams[2].draws, 1, "and for the Bees")
end)

t.test("stage5_finds_the_leader", function()
  t.eq(league.leader(sample()), "Owls", "7 points beats 3 and 0")
  local tie = {
    {name = "Bees", wins = 1, draws = 0, losses = 0},
    {name = "Moles", wins = 0, draws = 3, losses = 0},
  }
  t.eq(league.leader(tie), "Bees", "3 points each: the earlier team")
end)

t.test("stage6_builds_the_table_lines", function()
  t.deep_eq(league.table_lines(sample()), {
    "Owls 7 pts in 4 games",
    "Foxes 3 pts in 2 games",
    "Bees 0 pts in 0 games",
  })
end)
