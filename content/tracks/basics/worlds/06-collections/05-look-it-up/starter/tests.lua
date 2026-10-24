local t = require("oxide.test")
local list = require("price_list")

local prices = {milk = 1.2, bread = 2, eggs = 3.5}

t.test("looks_up_a_price", function()
  t.eq(list.price_of(prices, "milk"), 1.2)
  t.eq(list.price_of(prices, "eggs"), 3.5)
  t.is_nil(list.price_of(prices, "caviar"), "not on the price list")
end)

t.test("knows_which_items_exist", function()
  t.eq(list.sells(prices, "bread"), true)
  t.eq(list.sells(prices, "caviar"), false, "the shop does not sell caviar")
end)
