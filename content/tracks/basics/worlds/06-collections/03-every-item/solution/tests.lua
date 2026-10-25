local t = require("oxide.test")
local receipt = require("receipt")

t.test("totals_the_basket", function()
  t.eq(receipt.basket_total({2.5, 1, 4}), 7.5)
  t.eq(receipt.basket_total({3, 3, 3, 3}), 12)
  t.eq(receipt.basket_total({}), 0, "an empty basket costs nothing")
end)

t.test("counts_the_expensive_items", function()
  t.eq(receipt.count_over({2.5, 1, 4}, 2), 2, "2.5 and 4")
  t.eq(receipt.count_over({2.5, 1, 4}, 10), 0)
  t.eq(receipt.count_over({5, 5}, 5), 0, "5 is not above 5")
end)
