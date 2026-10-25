local t = require("oxide.test")
local sprint = require("sprint")

t.test("counts_down_to_go", function()
  t.eq(sprint.countdown(3), "3 2 1 go")
  t.eq(sprint.countdown(5), "5 4 3 2 1 go")
  t.eq(sprint.countdown(1), "1 go")
end)

t.test("a_break_every_15_minutes", function()
  t.eq(sprint.water_breaks(60), 4, "at 15, 30, 45 and 60")
  t.eq(sprint.water_breaks(50), 3, "at 15, 30 and 45")
  t.eq(sprint.water_breaks(10), 0, "too short for a break")
end)
