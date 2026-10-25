local t = require("oxide.test")
local plan = require("plan")

t.test("counts_the_weeks_to_the_goal", function()
  t.eq(plan.weeks_until(10), 6, "2, 3.5, 5, 6.5, 8, 9.5, 11: six weeks")
  t.eq(plan.weeks_until(5), 2, "2, 3.5, 5: two weeks")
  t.eq(plan.weeks_until(3), 1, "one week to pass 3 km")
end)

t.test("already_there_takes_no_weeks", function()
  t.eq(plan.weeks_until(2), 0, "you can already run 2 km")
  t.eq(plan.weeks_until(1), 0)
end)
