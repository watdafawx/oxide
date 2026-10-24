local t = require("oxide.test")
local pushups = require("pushups")

t.test("adds_up_every_day", function()
  t.eq(pushups.pushups_after(1), 1, "day 1 only")
  t.eq(pushups.pushups_after(3), 6, "1 + 2 + 3")
  t.eq(pushups.pushups_after(30), 465, "a whole month")
end)

t.test("zero_days_is_zero", function()
  t.eq(pushups.pushups_after(0), 0, "no days, no push-ups")
end)
