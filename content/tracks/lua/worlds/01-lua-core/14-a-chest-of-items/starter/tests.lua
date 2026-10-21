local t = require("oxide.test")
local p = require("pantry")

t.test("reads_a_count_or_zero", function()
  local pantry = {eggs = 12, ["brown-sugar"] = 3}
  t.eq(p.count_of(pantry, "eggs"), 12)
  t.eq(p.count_of(pantry, "brown-sugar"), 3)
  t.eq(p.count_of(pantry, "salt"), 0, "no such key means 0")
end)

t.test("puts_in_more", function()
  local pantry = {eggs = 12}
  p.put_in(pantry, "eggs", 6)
  p.put_in(pantry, "flour", 2)
  t.deep_eq(pantry, {eggs = 18, flour = 2})
end)

t.test("empties_out_a_key", function()
  local pantry = {eggs = 12, flour = 3}
  t.eq(p.empty_out(pantry, "eggs"), 12)
  t.deep_eq(pantry, {flour = 3}, "the key is gone, not set to 0")
  t.eq(p.empty_out(pantry, "salt"), 0)
end)
