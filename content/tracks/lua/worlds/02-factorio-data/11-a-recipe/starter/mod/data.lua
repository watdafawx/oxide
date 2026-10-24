-- Kit Mod: kit-alloy and the recipe that makes it.
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
    -- TODO: crafting takes 2 seconds; it eats 2 iron plates and 1 copper plate and makes 1 kit-alloy
    ingredients = {},
    results = {},
  },
})
