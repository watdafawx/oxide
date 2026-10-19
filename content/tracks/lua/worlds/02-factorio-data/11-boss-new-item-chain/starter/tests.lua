local t = require("oxide.test")
local mock = require("mock_factorio")

local function load() return mock.load_mod("mod", { validate = false }) end

-- amounts by ingredient name, e.g. { ["iron-plate"] = 2 }
local function amounts(list)
  local out = {}
  for _, e in ipairs(list or {}) do out[e.name] = e.amount end
  return out
end

local function unlocks(tech, recipe)
  for _, e in ipairs(tech and tech.effects or {}) do
    if e.type == "unlock-recipe" and e.recipe == recipe then return true end
  end
  return false
end

t.test("alloy_item_and_recipe_register", function()
  local raw = load()
  t.truthy(raw.item["kit-alloy"], "item kit-alloy is missing")
  mock.validate_prototype(raw.item["kit-alloy"])
  local r = raw.recipe["kit-alloy"]
  t.truthy(r, "recipe kit-alloy is missing")
  mock.validate_prototype(r)
  t.deep_eq(amounts(r.ingredients), { ["iron-plate"] = 2, ["copper-plate"] = 1 })
  t.deep_eq(amounts(r.results), { ["kit-alloy"] = 1 })
end)

t.test("crate_item_and_recipe_register", function()
  local raw = load()
  t.truthy(raw.item["kit-crate"], "item kit-crate is missing")
  mock.validate_prototype(raw.item["kit-crate"])
  t.eq(raw.item["kit-crate"].stack_size, 50)
  local r = raw.recipe["kit-crate"]
  t.truthy(r, "recipe kit-crate is missing")
  mock.validate_prototype(r)
  t.deep_eq(amounts(r.ingredients), { ["kit-alloy"] = 4, ["iron-gear-wheel"] = 1 })
  t.deep_eq(amounts(r.results), { ["kit-crate"] = 1 })
end)

t.test("entity_item_matches_place_result", function()
  local raw = load()
  local box = raw.container["kit-crate"]
  t.truthy(box, "container kit-crate is missing")
  mock.validate_prototype(box)
  t.eq(box.inventory_size, 48)
  t.truthy(raw.item["kit-crate"], "item kit-crate is missing")
  t.eq(raw.item["kit-crate"].place_result, "kit-crate", "the item must place the container")
  t.is_type(box.minable, "table", "the container needs `minable`")
  t.eq(box.minable.result, "kit-crate", "mining gives the item back")
end)

t.test("references_resolve", function()
  local raw = load()
  t.truthy(raw.recipe["kit-crate"] and raw.technology["kit-storage"], "finish the chain first")
  mock.validate_references()
end)

t.test("tech_chain_unlocks_both_recipes", function()
  local raw = load()
  t.truthy(unlocks(raw.technology["kit-metallurgy"], "kit-alloy"), "kit-metallurgy must unlock kit-alloy")
  t.truthy(unlocks(raw.technology["kit-storage"], "kit-crate"), "kit-storage must unlock kit-crate")
end)

t.test("prerequisites_form_a_chain", function()
  local raw = load()
  t.deep_eq(raw.technology["kit-metallurgy"].prerequisites, { "automation" })
  t.truthy(raw.technology["kit-storage"], "technology kit-storage is missing")
  t.deep_eq(raw.technology["kit-storage"].prerequisites, { "kit-metallurgy" })
end)

t.test("recipes_start_locked", function()
  local raw = load()
  t.truthy(raw.recipe["kit-crate"], "recipe kit-crate is missing")
  t.eq(raw.recipe["kit-alloy"].enabled, false, "kit-alloy must start disabled")
  t.eq(raw.recipe["kit-crate"].enabled, false, "kit-crate must start disabled")
end)

t.test("icons_are_valid", function()
  local raw = load()
  t.truthy(raw.technology["kit-storage"], "finish the chain first")
  mock.validate_all()   -- every prototype, vanilla fixture included
end)
