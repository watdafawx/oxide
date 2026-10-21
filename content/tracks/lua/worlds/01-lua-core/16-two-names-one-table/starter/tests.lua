local t = require("oxide.test")
local c = require("pantry_copy")

t.test("copy_has_the_same_contents", function()
  local pantry = {eggs = 12, flour = 3}
  local copy = c.copy_pantry(pantry)
  t.deep_eq(copy, {eggs = 12, flour = 3})
  t.falsy(copy == pantry, "a new table, not a second name")
end)

t.test("editing_the_copy_leaves_the_original", function()
  local pantry = {eggs = 12, flour = 3}
  local copy = c.copy_pantry(pantry)
  copy.eggs = 0
  copy.sugar = 5
  t.eq(pantry.eggs, 12, "the real pantry still has its eggs")
  t.is_nil(pantry.sugar, "and no new key")
end)
