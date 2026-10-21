local t = require("oxide.test")
local d = require("dish_eggs")

t.test("known_dishes_have_an_egg_count", function()
  t.eq(d.eggs_for("pancakes"), 3)
  t.eq(d.eggs_for("omelette"), 2)
  t.eq(d.eggs_for("cake"), 4)
end)

t.test("unknown_dish_needs_no_eggs", function()
  t.eq(d.eggs_for("soup"), 0)
  t.eq(d.eggs_for(""), 0)
end)
