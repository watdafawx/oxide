local t = require("oxide.test")
local recipes = require("recipes")

t.test("recipe_adds_up_its_items", function()
  local r = recipes.new("electronic-circuit", {1, 3})
  t.eq(r:total_items(), 4)
  t.eq(recipes.new("wooden-chest", {}):total_items(), 0)
end)

t.test("recipe_describes_itself", function()
  t.eq(recipes.new("electronic-circuit", {1, 3}):describe(), "electronic-circuit: 4 items")
  t.eq(recipes.new("pipe", {1}):describe(), "pipe: 1 items")
end)
