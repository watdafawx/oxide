local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

t.test("tech_unlocks_the_recipe", function()
  local tech = load().technology["kit-metallurgy"]
  t.truthy(tech, "technology kit-metallurgy is missing")
  local found = false
  for _, e in ipairs(tech.effects or {}) do
    if e.type == "unlock-recipe" and e.recipe == "kit-alloy" then found = true end
  end
  t.truthy(found, "add { type = \"unlock-recipe\", recipe = \"kit-alloy\" } to effects")
end)

t.test("prerequisites_exist", function()
  local raw = load()
  local tech = raw.technology["kit-metallurgy"]
  t.deep_eq(tech.prerequisites, { "automation" })
  mock.validate_references()
end)

t.test("unit_uses_valid_science_packs", function()
  local raw = load()
  local tech = raw.technology["kit-metallurgy"]
  t.is_type(tech.unit, "table", "add a `unit`")
  mock.validate_prototype(tech)
  t.eq(tech.unit.count, 30)
  t.eq(tech.unit.time, 15)
  t.deep_eq(tech.unit.ingredients, { { "automation-science-pack", 1 } })
  mock.validate_references()
end)

t.test("recipe_starts_disabled", function()
  t.eq(load().recipe["kit-alloy"].enabled, false, "set enabled = false")
end)
