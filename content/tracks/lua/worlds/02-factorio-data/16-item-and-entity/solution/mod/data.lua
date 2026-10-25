-- Kit Mod: the kit-crate chest.
data:extend({
  {
    type = "item",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    stack_size = 50,
    subgroup = "storage",
    order = "a[items]-z[kit-crate]",
    place_result = "kit-crate",
  },
  {
    type = "container",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    flags = { "placeable-neutral", "player-creation" },
    inventory_size = 48,
    minable = { mining_time = 0.2, result = "kit-crate" },
    collision_box = { { -0.35, -0.35 }, { 0.35, 0.35 } },
    selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },
  },
})
