local t = require("oxide.test")
local tips = require("tooltips")

t.test("labels_an_ingredient", function()
  t.eq(tips.ingredient_label(2, "iron-plate"), "2x iron-plate")
  t.eq(tips.ingredient_label(10, "copper-cable"), "10x copper-cable")
end)

t.test("describes_crafting_time", function()
  t.eq(tips.crafting_line("iron-gear-wheel", 0.5), "iron-gear-wheel: 0.5s")
  t.eq(tips.crafting_line("steel-plate", 16), "steel-plate: 16s")
end)
