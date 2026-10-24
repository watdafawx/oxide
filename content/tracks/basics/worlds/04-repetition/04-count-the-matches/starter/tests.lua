local t = require("oxide.test")
local month = require("month")

t.test("a_week_has_six_running_days", function()
  t.eq(month.running_days(7), 6, "day 7 is a rest day")
  t.eq(month.running_days(6), 6, "no rest day yet")
  t.eq(month.running_days(0), 0, "no days at all")
end)

t.test("a_month_has_26_running_days", function()
  t.eq(month.running_days(30), 26, "rest on days 7, 14, 21 and 28")
end)
