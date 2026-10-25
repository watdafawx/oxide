-- Kit Mod: kit-alloy, unlocked by research.
data:extend({
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
