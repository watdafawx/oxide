local t = require("oxide.test")
local order = require("order")

t.test("food_price_counts_toppings", function()
  t.eq(order.food_price(2, 2), 20, "2 pizzas with 2 toppings")
  t.eq(order.food_price(4, 0), 28, "4 plain pizzas")
end)

t.test("discount_over_50", function()
  t.eq(order.discount(60), 6, "a tenth of 60")
  t.eq(order.discount(50), 0, "exactly 50 is not over 50")
  t.eq(order.discount(20), 0)
end)

t.test("order_total_uses_every_rule", function()
  t.eq(order.order_total(4, 2, 5), 44, "40 of food, no discount, 4 for delivery")
  t.eq(order.order_total(6, 2, 2), 56, "60 of food, 6 off, 2 for delivery")
end)
