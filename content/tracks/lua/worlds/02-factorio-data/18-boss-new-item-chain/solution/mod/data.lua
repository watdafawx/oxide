-- Kit Mod. Chain: kit-alloy -> kit-crate (item + chest), behind two technologies.

data:extend({
  -- Stage 1 (done): the alloy
  {
    type = "item",
    name = "kit-alloy",
    icon = "__base__/graphics/icons/iron-plate.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
  {
    type = "recipe",
    name = "kit-alloy",
    enabled = false,
    energy_required = 2,
    ingredients = {
      { type = "item", name = "iron-plate", amount = 2 },
      { type = "item", name = "copper-plate", amount = 1 },
    },
    results = { { type = "item", name = "kit-alloy", amount = 1 } },
  },
  {
    type = "technology",
    name = "kit-metallurgy",
    icon = "__base__/graphics/technology/steel-processing.png",
    icon_size = 256,
    prerequisites = { "automation" },
    effects = { { type = "unlock-recipe", recipe = "kit-alloy" } },
    unit = { count = 30, time = 15, ingredients = { { "automation-science-pack", 1 } } },
  },
})

data:extend({
  {
    type = "item",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    stack_size = 50,
    subgroup = "storage",
    order = "a[items]-z[kit-crate]",
    place_result = "kit-crate",
  },
  {
    type = "recipe",
    name = "kit-crate",
    enabled = false,
    energy_required = 3,
    ingredients = {
      { type = "item", name = "kit-alloy", amount = 4 },
      { type = "item", name = "iron-gear-wheel", amount = 1 },
    },
    results = { { type = "item", name = "kit-crate", amount = 1 } },
  },
  {
    type = "container",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    flags = { "placeable-neutral", "player-creation" },
    inventory_size = 48,
    minable = { mining_time = 0.2, result = "kit-crate" },
    collision_box = { { -0.35, -0.35 }, { 0.35, 0.35 } },
    selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },
  },
  {
    type = "technology",
    name = "kit-storage",
    icon = "__base__/graphics/technology/steel-processing.png",
    icon_size = 256,
    prerequisites = { "kit-metallurgy" },
    effects = { { type = "unlock-recipe", recipe = "kit-crate" } },
    unit = { count = 50, time = 20, ingredients = { { "automation-science-pack", 1 } } },
  },
})
