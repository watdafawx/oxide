local t = require("oxide.test")
local closing = require("closing")

t.test("counts_every_item_in_stock", function()
  t.eq(closing.items_in_stock({milk = 3, eggs = 12, bread = 5}), 20)
  t.eq(closing.items_in_stock({}), 0, "empty shelves")
end)

t.test("values_the_stock", function()
  local prices = {milk = 1, eggs = 0.25, bread = 2}
  t.eq(closing.stock_value({milk = 3, eggs = 12, bread = 5}, prices), 16, "3 + 3 + 10")
  t.eq(closing.stock_value({bread = 1}, prices), 2)
end)
