local t = require("oxide.test")
local c = require("recipe_copy")

local function gear()
  return {name = "iron-gear-wheel", time = 0.5, ingredients = {{"iron-plate", 2}}}
end

t.test("copies_nested_tables", function()
  local recipe = gear()
  local copy = c.deep_copy(recipe)
  t.deep_eq(copy, gear(), "same contents")
  t.falsy(copy == recipe, "a new outer table")
  t.falsy(copy.ingredients == recipe.ingredients, "a new ingredient list")
  t.falsy(copy.ingredients[1] == recipe.ingredients[1], "and a new pair inside it")
  t.eq(c.deep_copy(7), 7, "plain values come back as they are")
end)

t.test("editing_the_copy_leaves_the_original", function()
  local recipe = gear()
  local copy = c.deep_copy(recipe)
  copy.ingredients[1][2] = 99
  copy.name = "steel-gear"
  t.eq(recipe.ingredients[1][2], 2, "the original ingredient is untouched")
  t.eq(recipe.name, "iron-gear-wheel")
end)
