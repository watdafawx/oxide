local t = require("oxide.test")
local pizza = require("pizza")

t.test("defines_base_price", function()
  t.is_type(pizza.base_price, "function", "M.base_price should be a function")
  t.eq(pizza.base_price(), 7)
end)

t.test("defines_topping_price", function()
  t.is_type(pizza.topping_price, "function", "M.topping_price should be a function")
  t.eq(pizza.topping_price(), 1.5)
end)
