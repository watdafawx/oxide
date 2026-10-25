local t = require("oxide.test")
local painter = require("painter")

t.test("paint_changes_the_picture_it_is_given", function()
  local picture = {"white", "white", "white"}
  painter.paint(picture, 2, "red")
  t.deep_eq(picture, {"white", "red", "white"}, "the caller's own picture changed")
end)

t.test("snapshot_is_a_separate_copy", function()
  local picture = {"white", "blue", "white"}
  local backup = painter.snapshot(picture)
  t.deep_eq(backup, {"white", "blue", "white"}, "the copy has the same colours")
  t.ne(backup, picture, "the copy must be a new table, not the same one")
  backup[1] = "black"
  t.eq(picture[1], "white", "changing the copy must leave the picture alone")
end)
