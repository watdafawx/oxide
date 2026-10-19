local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("copy_is_independent", function()
  local raw = load()
  local item, recipe = raw.item["kit-gear"], raw.recipe["kit-gear"]
  t.truthy(item and recipe, "extend an item and a recipe named kit-gear")
  t.ne(item, raw.item["iron-gear-wheel"], "kit-gear is the vanilla table itself: copy it")
  t.ne(recipe, raw.recipe["iron-gear-wheel"], "the recipe is the vanilla table itself: copy it")
  t.ne(recipe.ingredients, raw.recipe["iron-gear-wheel"].ingredients, "ingredients is shared with vanilla")
  t.eq(item.stack_size, 200)
  t.eq(recipe.ingredients[1].amount, 4)
  t.eq(recipe.results[1].name, "kit-gear")
end)

t.test("original_is_untouched", function()
  local raw = load()
  local vanilla = mock.pristine()
  t.deep_eq(raw.item["iron-gear-wheel"], vanilla.item["iron-gear-wheel"], "vanilla gear item changed")
  t.deep_eq(raw.recipe["iron-gear-wheel"], vanilla.recipe["iron-gear-wheel"], "vanilla gear recipe changed")
end)

t.test("new_name_is_unique", function()
  local raw = load()
  t.eq(raw.item["iron-gear-wheel"].name, "iron-gear-wheel", "vanilla item was renamed")
  t.eq(raw.recipe["iron-gear-wheel"].name, "iron-gear-wheel", "vanilla recipe was renamed")
  t.eq(raw.item["kit-gear"].name, "kit-gear")
  t.eq(raw.recipe["kit-gear"].name, "kit-gear")
  t.deep_eq(mock.overwritten(), {}, "a prototype with the same type and name was replaced")
  mock.validate_references()
end)
