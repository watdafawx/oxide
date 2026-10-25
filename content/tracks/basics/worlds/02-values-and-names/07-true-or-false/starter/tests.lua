local t = require("oxide.test")
local checks = require("checks")

t.test("sold_out_only_at_zero", function()
  t.eq(checks.sold_out(0), true, "no seats left")
  t.eq(checks.sold_out(1), false, "one seat is still free")
  t.eq(checks.sold_out(80), false)
end)

t.test("late_show_is_15_and_up", function()
  t.eq(checks.old_enough(15), true, "15 is old enough")
  t.eq(checks.old_enough(14), false, "14 is not")
  t.eq(checks.old_enough(40), true)
end)
