**Why now:** the Kit Mod has items and recipes but nothing to put on the ground. A new chest is the smallest real **entity**, and it links two prototypes in a loop: an item that places it and an entity that gives the item back.

An entity is another prototype, here `type = "container"`. Only `inventory_size` is container-specific and mandatory; the rest comes from the entity base:

```lua
{
  type = "container",
  name = "kit-crate",
  icon = "__base__/graphics/icons/wooden-chest.png",   -- needed once the entity is placeable
  flags = { "placeable-neutral", "player-creation" },
  inventory_size = 48,
  minable = { mining_time = 0.2, result = "kit-crate" },
  collision_box = { { -0.35, -0.35 }, { 0.35, 0.35 } },   -- what blocks walking and building
  selection_box = { { -0.5, -0.5 }, { 0.5, 0.5 } },       -- what the mouse can hit
}
```

- The **item** says `place_result = "kit-crate"`; that is the only link from item to entity.
- `minable.result` names the item you get back. Without `minable` the entity cannot be picked up.
- `collision_box` defaults to an empty box (no collision), so give it one. Centred entities use a box that mirrors around 0, which keeps them aligned to the grid.
- The `flags` entry `placeable-neutral` is what the docs tie the mandatory icon to; `player-creation` is the usual companion.
- `mock_factorio` checks that `place_result` names an entity and `minable.result` names an item: loops that point nowhere are the classic first-mod bug.

## Your turn

- Open `mod/data.lua`. The item is half done; fill in the `kit-crate` container and the item's link to it.
