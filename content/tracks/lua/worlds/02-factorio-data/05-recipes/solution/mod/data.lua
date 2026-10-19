-- Kit Mod, step 3: a recipe for kit-alloy.
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
    energy_required = 2,
    ingredients = {
      { type = "item", name = "iron-plate", amount = 2 },
      { type = "item", name = "copper-plate", amount = 1 },
    },
    results = { { type = "item", name = "kit-alloy", amount = 1 } },
  },
})
