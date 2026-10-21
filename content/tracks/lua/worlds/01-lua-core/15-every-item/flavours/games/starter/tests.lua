local t = require("oxide.test")
local scan = require("chest_scan")

t.test("totals_the_stock", function()
  t.eq(scan.total_items({coal = 12, stone = 3, wood = 5}), 20)
  t.eq(scan.total_items({}), 0)
end)

t.test("clears_empty_entries", function()
  local chest = {coal = 12, stone = 0, wood = 0, ["iron-plate"] = 1}
  local same = scan.remove_empty(chest)
  t.deep_eq(chest, {coal = 12, ["iron-plate"] = 1}, "zeros are gone")
  t.truthy(same == chest, "the same table, edited in place")
end)
