local t = require("oxide.test")
local gate = require("gate")

t.test("short_riders_are_stopped", function()
  t.eq(gate.gate_message(110), "Too short, sorry", "110 cm is too short")
  t.eq(gate.gate_message(150), "Enjoy the ride", "150 cm may ride")
  t.eq(gate.gate_message(120), "Enjoy the ride", "exactly 120 cm may ride")
end)

t.test("children_pay_half", function()
  t.eq(gate.fast_pass_price(8), 5, "an 8-year-old pays 5")
  t.eq(gate.fast_pass_price(30), 10, "an adult pays 10")
  t.eq(gate.fast_pass_price(12), 10, "12 is no longer under 12")
end)
