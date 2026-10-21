local t = require("oxide.test")
local totals = require("ingredient_totals")

t.test("totals_every_count", function()
  t.eq(totals.total_ingredients({2, 1, 3}), 6)
  t.eq(totals.total_ingredients({10}), 10)
  t.eq(totals.total_ingredients({}), 0)
end)

t.test("counts_the_bulk_ingredients", function()
  t.eq(totals.bulk_ingredients({1, 5, 20, 8}, 6), 2)
  t.eq(totals.bulk_ingredients({1, 2}, 6), 0)
  t.eq(totals.bulk_ingredients({}, 6), 0)
end)
