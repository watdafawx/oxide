local t = require("oxide.test")
local g = require("gear_split")

t.test("seven_plates_make_three_and_a_half", function()
  t.eq(g.gears_exact(7), 3.5)
  t.eq(g.gears_exact(8), 4)
end)

t.test("whole_gears_round_down", function()
  t.eq(g.whole_gears(7), 3, "the leftover plate is not a fourth gear")
  t.eq(g.whole_gears(8), 4)
  t.eq(g.whole_gears(1), 0)
end)
