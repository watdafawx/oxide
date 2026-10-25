local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("recipe_starts_locked", function()
  t.eq(load().recipe["kit-alloy"].enabled, false, "set enabled = false on the recipe")
end)

t.test("tech_unlocks_the_recipe", function()
  local tech = load().technology["kit-metallurgy"]
  t.truthy(tech, "technology kit-metallurgy is missing")
  local found = false
  for _, e in ipairs(tech.effects or {}) do
    if e.type == "unlock-recipe" and e.recipe == "kit-alloy" then found = true end
  end
  t.truthy(found, "add { type = \"unlock-recipe\", recipe = \"kit-alloy\" } to effects")
  mock.validate_references()
end)

t.test("prerequisites_exist", function()
  local tech = load().technology["kit-metallurgy"]
  t.deep_eq(tech.prerequisites, { "automation" })
  mock.validate_references()
end)
