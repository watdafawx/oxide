local t = require("oxide.test")
local o = require("omelettes")

t.test("seven_eggs_make_three_and_a_half", function()
  t.eq(o.omelettes_exact(7), 3.5)
  t.eq(o.omelettes_exact(8), 4)
end)

t.test("whole_omelettes_round_down", function()
  t.eq(o.whole_omelettes(7), 3, "the leftover egg is not a fourth omelette")
  t.eq(o.whole_omelettes(8), 4)
  t.eq(o.whole_omelettes(1), 0)
end)
