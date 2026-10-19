-- Kit Mod, step 5: research kit-alloy.
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
    -- TODO: lock it until researched
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
    -- TODO: prerequisites = { "automation" }
    -- TODO: effects (unlock-recipe kit-alloy)
    -- TODO: unit = 30 units, 15 s each, 1 automation-science-pack per unit
  },
})
