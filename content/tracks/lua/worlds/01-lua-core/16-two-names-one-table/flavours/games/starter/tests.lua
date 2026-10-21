local t = require("oxide.test")
local c = require("chest_copy")

t.test("copy_has_the_same_contents", function()
  local chest = {coal = 12, stone = 3}
  local copy = c.copy_chest(chest)
  t.deep_eq(copy, {coal = 12, stone = 3})
  t.falsy(copy == chest, "a new table, not a second name")
end)

t.test("editing_the_copy_leaves_the_original", function()
  local chest = {coal = 12, stone = 3}
  local copy = c.copy_chest(chest)
  copy.coal = 0
  copy.wood = 5
  t.eq(chest.coal, 12, "the real chest still has its coal")
  t.is_nil(chest.wood, "and no new key")
end)
