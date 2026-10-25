local t = require("oxide.test")
local badges = require("badges")

t.test("finds_the_first_day", function()
  t.eq(badges.first_day_over(10), 15, "day 14 is exactly 10 km, day 15 is 10.5")
  t.eq(badges.first_day_over(5), 5, "day 5 is 5.5 km")
  t.eq(badges.first_day_over(1), 1, "day 1 already beats 1 km")
end)

t.test("nil_when_no_day_matches", function()
  t.is_nil(badges.first_day_over(100), "no day this month reaches 100 km")
end)
