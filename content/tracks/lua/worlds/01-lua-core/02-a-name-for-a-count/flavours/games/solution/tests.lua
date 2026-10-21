local t = require("oxide.test")
local plan = require("gear_plan")

t.test("wants_five_gears", function()
  t.eq(plan.gears_wanted(), 5, "the plan is 5 gears")
end)

t.test("five_gears_need_10_plates", function()
  t.eq(plan.plates_needed(), 10, "5 gears, 2 plates each")
end)
