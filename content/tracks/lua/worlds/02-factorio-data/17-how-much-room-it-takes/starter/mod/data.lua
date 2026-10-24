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
    -- TODO: this box starts at the centre instead of around it, and there is no selection box.
    --       Centre the collision box a little inside the tile, and add a selection box for the whole tile.
    collision_box = { { 0, 0 }, { 0.7, 0.7 } },
  },
})
