-- Kit Mod: a row of its own in the crafting menu, and the alloy.
data:extend({
  { type = "item-subgroup", name = "kit-parts", group = "intermediate-products", order = "z" },
  {
    type = "item",
    name = "kit-alloy",
    icon = "__base__/graphics/icons/iron-plate.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
})
