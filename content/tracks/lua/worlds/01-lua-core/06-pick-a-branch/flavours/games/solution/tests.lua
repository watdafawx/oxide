local t = require("oxide.test")
local i = require("item_plates")

t.test("known_items_have_a_plate_count", function()
  t.eq(i.plates_for("iron-gear-wheel"), 2)
  t.eq(i.plates_for("pipe"), 1)
  t.eq(i.plates_for("steel-plate"), 5)
end)

t.test("unknown_item_needs_no_plates", function()
  t.eq(i.plates_for("wooden-chest"), 0)
  t.eq(i.plates_for(""), 0)
end)
