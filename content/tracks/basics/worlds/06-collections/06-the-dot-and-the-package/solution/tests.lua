local t = require("oxide.test")
local products = require("products")

t.test("adds_and_changes_prices", function()
  local prices = {milk = 1.2}
  products.set_price(prices, "eggs", 3)
  products.set_price(prices, "milk", 1.5)
  t.deep_eq(prices, {milk = 1.5, eggs = 3}, "eggs added, milk changed")
end)

t.test("removes_a_product", function()
  local prices = {milk = 1.2, eggs = 3}
  products.discontinue(prices, "eggs")
  t.deep_eq(prices, {milk = 1.2}, "only milk is left")
  t.is_nil(prices.eggs)
end)
