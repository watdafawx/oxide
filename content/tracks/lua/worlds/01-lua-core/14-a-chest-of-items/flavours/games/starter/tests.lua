local t = require("oxide.test")
local c = require("chest")

t.test("reads_a_count_or_zero", function()
  local chest = {coal = 12, ["iron-plate"] = 3}
  t.eq(c.count_of(chest, "coal"), 12)
  t.eq(c.count_of(chest, "iron-plate"), 3)
  t.eq(c.count_of(chest, "stone"), 0, "no such key means 0")
end)

t.test("puts_in_more", function()
  local chest = {coal = 12}
  c.put_in(chest, "coal", 6)
  c.put_in(chest, "stone", 2)
  t.deep_eq(chest, {coal = 18, stone = 2})
end)

t.test("empties_out_a_key", function()
  local chest = {coal = 12, stone = 3}
  t.eq(c.empty_out(chest, "coal"), 12)
  t.deep_eq(chest, {stone = 3}, "the key is gone, not set to 0")
  t.eq(c.empty_out(chest, "wood"), 0)
end)
