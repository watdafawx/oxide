local t = require("oxide.test")
local delivery = require("delivery")

t.test("fee_is_returned_not_printed", function()
  t.eq(delivery.delivery_fee(2), 2, "2 km: the basic fee")
  t.eq(delivery.delivery_fee(5), 4, "5 km: 2, plus 1 for each of the last 2 km")
end)

t.test("total_adds_the_fee", function()
  t.eq(delivery.order_total(20, 5), 24, "20 of food, 4 for delivery")
  t.eq(delivery.order_total(14, 1), 16, "14 of food, 2 for delivery")
end)
