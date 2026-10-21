local t = require("oxide.test")
local shop = require("shopping")

t.test("totals_every_count", function()
  t.eq(shop.total_items({2, 1, 3}), 6)
  t.eq(shop.total_items({10}), 10)
  t.eq(shop.total_items({}), 0)
end)

t.test("counts_the_bulk_buys", function()
  t.eq(shop.bulk_items({1, 5, 20, 8}, 6), 2)
  t.eq(shop.bulk_items({1, 2}, 6), 0)
  t.eq(shop.bulk_items({}, 6), 0)
end)
