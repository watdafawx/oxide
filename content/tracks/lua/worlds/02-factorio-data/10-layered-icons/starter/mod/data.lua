-- Kit Mod: kit-alloy with a layered icon.
data:extend({
  {
    type = "item",
    name = "kit-alloy",
    -- TODO: replace this with an icons list: the Kit Mod's own picture at the bottom,
    --       then vanilla's iron plate on top with a coloured tint
    icon = "__kit-mod__/graphics/icons/kit-alloy.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
})
