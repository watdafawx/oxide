local t = require("oxide.test")
local basket = require("basket")

t.test("first_and_count", function()
  t.eq(basket.first_item({2.5, 1, 4}), 2.5)
  t.eq(basket.item_count({2.5, 1, 4}), 3)
  t.eq(basket.item_count({}), 0, "an empty basket")
end)

t.test("second_to_last", function()
  t.eq(basket.second_to_last({2.5, 1, 4}), 1)
  t.eq(basket.second_to_last({3, 8, 0.5, 6, 2}), 6, "five items")
end)
