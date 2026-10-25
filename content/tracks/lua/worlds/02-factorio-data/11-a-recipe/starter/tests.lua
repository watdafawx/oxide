local t = require("oxide.test")
local mock = require("mock_factorio")

local function recipe()
  local r = mock.load_mod("mod", { validate = false }).recipe["kit-alloy"]
  t.truthy(r, "data:extend a recipe named kit-alloy")
  return r
end

t.test("recipe_ingredients_are_item_tables", function()
  local r = recipe()
  t.is_type(r.ingredients, "table", "add an ingredients list")
  local amounts = {}
  for i, ing in ipairs(r.ingredients) do
    t.is_type(ing, "table", "ingredient " .. i .. " must be a table")
    t.eq(ing.type, "item", "ingredient " .. i .. " needs type = \"item\"")
    amounts[ing.name] = ing.amount
  end
  t.deep_eq(amounts, { ["iron-plate"] = 2, ["copper-plate"] = 1 })
end)

t.test("recipe_result_is_the_item", function()
  local r = recipe()
  t.is_nil(r.result, "`result` is the pre-2.0 field, use `results`")
  t.deep_eq(r.results, { { type = "item", name = "kit-alloy", amount = 1 } })
  mock.validate_references()
end)

t.test("crafting_time_is_two_seconds", function()
  local r = recipe()
  t.eq(r.energy_required, 2)
  mock.validate_prototype(r)   -- the message names an old-style entry
end)
