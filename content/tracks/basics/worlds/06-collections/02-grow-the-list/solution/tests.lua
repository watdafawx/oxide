local t = require("oxide.test")
local till = require("till")

t.test("scanning_adds_to_the_end", function()
  local basket = {}
  till.scan(basket, 2.5)
  till.scan(basket, 1)
  t.deep_eq(basket, {2.5, 1}, "two scans, in order")
  t.eq(#basket, 2)
end)

t.test("a_price_can_be_corrected", function()
  local basket = {2.5, 10, 4}
  till.fix_price(basket, 2, 1)
  t.deep_eq(basket, {2.5, 1, 4}, "slot 2 fixed, the others untouched")
end)
