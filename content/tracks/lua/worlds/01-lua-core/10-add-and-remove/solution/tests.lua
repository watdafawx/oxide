local t = require("oxide.test")
local shop = require("shopping_list")

t.test("adds_to_the_end", function()
  local list = {"tea"}
  shop.add_item(list, "milk")
  shop.add_item(list, "sugar")
  t.deep_eq(list, {"tea", "milk", "sugar"})
  t.eq(#list, 3)
end)

t.test("crossing_off_closes_the_gap", function()
  local list = {"tea", "milk", "sugar"}
  t.eq(shop.cross_off(list, 2), "milk", "hands back what it removed")
  t.deep_eq(list, {"tea", "sugar"}, "sugar moved down")
  t.eq(#list, 2, "no hole left behind")
end)
