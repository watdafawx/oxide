-- Kit Mod: the kit-crate chest.
data:extend({
  {
    type = "item",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    stack_size = 50,
    subgroup = "storage",
    order = "a[items]-z[kit-crate]",
    -- TODO: name the entity this item puts down
  },
  {
    type = "container",
    name = "kit-crate",
    -- TODO: the wooden chest's icon, the two flags vanilla chests use, and 48 slots
    -- TODO: mining takes 0.2 seconds and gives back the kit-crate item
    collision_box = { { -0.35, -0.35 }, { 0.35, 0.35 } },
    selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },
  },
})
