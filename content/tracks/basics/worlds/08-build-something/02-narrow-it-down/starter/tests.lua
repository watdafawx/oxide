local t = require("oxide.test")
local league = require("league")

t.test("points_are_3_per_win_and_1_per_draw", function()
  t.eq(league.points(2, 1), 7, "2 wins and 1 draw")
  t.eq(league.points(0, 4), 4, "4 draws")
end)

t.test("played_counts_every_game", function()
  t.eq(league.played(2, 1, 1), 4, "2 wins, 1 draw, 1 loss")
  t.eq(league.played(0, 0, 3), 3, "3 losses")
end)

t.test("table_line_reads_right", function()
  t.eq(league.table_line("Owls", 2, 1, 1), "Owls 7 pts in 4 games")
end)
