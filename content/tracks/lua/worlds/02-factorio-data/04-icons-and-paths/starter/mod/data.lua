-- Kit Mod, step 2: a proper icon.
data:extend({
  {
    type = "item",
    name = "kit-alloy",
    -- TODO: replace this with `icons`: layer 1 = __kit-mod__/graphics/icons/kit-alloy.png,
    --       layer 2 = __base__/graphics/icons/iron-plate.png with a tint
    icon = "graphics/icons/kit-alloy.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
})
