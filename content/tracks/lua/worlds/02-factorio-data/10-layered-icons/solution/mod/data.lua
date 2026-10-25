-- Kit Mod: kit-alloy with a layered icon.
data:extend({
  {
    type = "item",
    name = "kit-alloy",
    icons = {
      { icon = "__kit-mod__/graphics/icons/kit-alloy.png" },
      { icon = "__base__/graphics/icons/iron-plate.png", tint = { r = 0.9, g = 0.5, b = 0.2, a = 0.8 } },
    },
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
})
