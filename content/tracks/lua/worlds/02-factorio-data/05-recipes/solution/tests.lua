local t = require("oxide.test")
local mock = require("mock_factorio")

local function recipe()
  local r = mock.load_mod("mod", { validate = false }).recipe["kit-alloy"]
  t.truthy(r, "data:extend a recipe named kit-alloy")
  return r
end

t.test("recipe_ingredients_use_2_0_format", function()
  local r = recipe()
  mock.validate_prototype(r)   -- the message names an old-style entry
  local amounts = {}
  for _, ing in ipairs(r.ingredients) do
    t.eq(ing.type, "item")
    amounts[ing.name] = ing.amount
  end
  t.deep_eq(amounts, { ["iron-plate"] = 2, ["copper-plate"] = 1 })
end)

t.test("recipe_result_is_the_item", function()
  local r = recipe()
  t.is_nil(r.result, "`result` is the old field, use `results`")
  t.deep_eq(r.results, { { type = "item", name = "kit-alloy", amount = 1 } })
  mock.validate_references()
end)

t.test("crafting_time_is_two_seconds", function()
  local r = recipe()
  t.eq(r.energy_required, 2)
  mock.validate_prototype(r)
end)
