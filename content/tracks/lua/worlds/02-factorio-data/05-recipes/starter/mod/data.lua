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
    -- TODO: this is the pre-2.0 shape. Use { type = "item", name = ..., amount = ... } tables,
    --       `results` instead of `result`, and energy_required = 2.
    ingredients = { { "iron-plate", 2 }, { "copper-plate", 1 } },
    result = "kit-alloy",
  },
})
