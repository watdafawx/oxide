-- Kit Mod, step 7: the kit-crate chest.
data:extend({
  {
    type = "item",
    name = "kit-crate",
    icon = "__base__/graphics/icons/wooden-chest.png",
    stack_size = 50,
    subgroup = "storage",
    order = "a[items]-z[kit-crate]",
    -- TODO: place_result = the entity this item puts down
  },
  {
    type = "container",
    name = "kit-crate",
    -- TODO: icon (the wooden-chest one), the two flags that let a player place it, 48 inventory slots
    -- TODO: minable (0.2 s, gives back the kit-crate item)
    -- TODO: collision_box and selection_box, centred on 0
  },
})
