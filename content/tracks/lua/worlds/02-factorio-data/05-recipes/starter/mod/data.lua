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
    -- TODO: this is the pre-2.0 shape. Make every ingredient a table with a type, a name and an
    --       amount; put the product in a `results` list of the same kind of table instead of
    --       `result`; and make crafting take 2 seconds.
    ingredients = { { "iron-plate", 2 }, { "copper-plate", 1 } },
    result = "kit-alloy",
  },
})
