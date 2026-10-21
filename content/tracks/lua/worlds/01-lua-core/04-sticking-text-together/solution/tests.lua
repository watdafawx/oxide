local t = require("oxide.test")
local labels = require("labels")

t.test("labels_an_ingredient", function()
  t.eq(labels.ingredient_label(3, "eggs"), "3 eggs")
  t.eq(labels.ingredient_label(500, "flour"), "500 flour")
end)

t.test("describes_cooking_time", function()
  t.eq(labels.cooking_line("pancakes", 12), "pancakes: 12 min")
  t.eq(labels.cooking_line("rice", 0.5), "rice: 0.5 min")
end)
