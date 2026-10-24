The Kit Mod has parts but nothing to put on the ground. A chest of your own, `kit-crate`, is the smallest real entity, and it needs two prototypes that point at each other.

**Two prototypes, one name.** In your pocket the chest is an item; on the ground it is an entity. They are two separate prototypes that share a name: `data.raw.item["kit-crate"]` and `data.raw.container["kit-crate"]`.

**Item to entity.** The item's `place_result` names the entity it puts down when you click the ground. It is the only link from the item to the entity.

**Entity back to item.** The entity's `minable` says how to pick it up again: `mining_time` in seconds and `result`, the item you get back. Without `minable`, a placed chest could never be removed.

**A container.** `type = "container"` is the entity kind for chests. The one container field you must write is `inventory_size`, the number of slots.

**Flags.** `flags` is a list of switches, each named by a string; a switch is on when its name is in the list. `"placeable-neutral"` lets the entity be placed on the map, and once it can be placed the docs make `icon` mandatory. Vanilla chests also carry `"player-creation"`, which marks them as something players build.

```lua
{
  type = "container",
  name = "kit-bin",
  icon = "__base__/graphics/icons/wooden-chest.png",
  flags = { "placeable-neutral", "player-creation" },
  inventory_size = 16,
  minable = { mining_time = 0.5, result = "kit-bin" },
}
```

## Your turn

- Give the `kit-crate` item a `place_result` that names the container.
- Give the container an icon, the two flags and 48 slots.
- Make mining take 0.2 seconds and give back the `kit-crate` item.
