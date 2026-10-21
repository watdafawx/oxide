local t = require("oxide.test")
local recipes = require("recipes")

t.test("recipe_adds_up_its_items", function()
  local r = recipes.new("pancakes", {3, 1, 2})
  t.eq(r:total_items(), 6)
  t.eq(recipes.new("tea", {}):total_items(), 0)
end)

t.test("recipe_describes_itself", function()
  t.eq(recipes.new("pancakes", {3, 1, 2}):describe(), "pancakes: 6 items")
  t.eq(recipes.new("toast", {2}):describe(), "toast: 2 items")
end)
