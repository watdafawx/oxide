local t = require("oxide.test")
local log = require("log")

t.test("skips_the_ill_days", function()
  t.eq(log.average_km(7), 6, "6, 4, 8 and 6, with the ill days left out")
  t.eq(log.average_km(4), 5, "6 and 4")
  t.eq(log.average_km(3), 6, "only day 3 counts")
end)

t.test("all_ill_means_zero", function()
  t.eq(log.average_km(2), 0, "days 1 and 2 were both ill days")
end)
