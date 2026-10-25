local t = require("oxide.test")
local discounts = require("discounts")

t.test("staff_pay_7", function()
  t.eq(discounts.staff_price(), 7, "9 minus the staff discount of 2")
end)

t.test("matinee_costs_6", function()
  t.eq(discounts.matinee_price(), 6, "the matinee price")
end)
