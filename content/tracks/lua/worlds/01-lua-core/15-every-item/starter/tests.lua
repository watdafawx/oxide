local t = require("oxide.test")
local scan = require("pantry_scan")

t.test("totals_the_stock", function()
  t.eq(scan.total_items({eggs = 12, flour = 3, milk = 5}), 20)
  t.eq(scan.total_items({}), 0)
end)

t.test("clears_empty_entries", function()
  local pantry = {eggs = 12, flour = 0, milk = 0, sugar = 1}
  local same = scan.remove_empty(pantry)
  t.deep_eq(pantry, {eggs = 12, sugar = 1}, "zeros are gone")
  t.truthy(same == pantry, "the same table, edited in place")
end)
