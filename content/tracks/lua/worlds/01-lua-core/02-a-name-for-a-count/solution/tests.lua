local t = require("oxide.test")
local plan = require("pancake_plan")

t.test("plans_four_batches", function()
  t.eq(plan.planned_batches(), 4, "the plan is 4 batches")
end)

t.test("four_batches_need_12_eggs", function()
  t.eq(plan.eggs_needed(), 12, "4 batches, 3 eggs each")
end)
