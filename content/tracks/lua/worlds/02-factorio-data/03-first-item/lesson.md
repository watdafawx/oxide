**Why now:** the calculator needs something new to craft. In the game, "something new" means one more row in `data.raw.item`, added with `data:extend`.

`data:extend` takes an **array** of prototype tables. Every prototype needs a `type` and a `name`. An `item` also needs a `stack_size` (a whole number of at least 1) and an `icon`:

```lua
data:extend({
  {
    type = "item",
    name = "kit-alloy",
    icon = "__base__/graphics/icons/iron-plate.png",   -- root + path, forward slashes
    stack_size = 100,
    subgroup = "intermediate-product",   -- where it shows in the inventory
    order = "z[kit-alloy]",              -- sort key inside the subgroup
  },
})
```

- `icon_size` defaults to 64, so you rarely write it.
- `subgroup` and `order` are optional in the docs, but without them the item lands in an unsorted corner. Quests in this world always set them.
- Note the `{ ... }` inside `({ ... })`: passing a single table without the outer array raises "Expected array of prototypes".
- The game checks mandatory fields when the stage ends, not at `data:extend`. Oxide does the same.

## Your turn

- Open `mod/data.lua`.
- Finish the `kit-alloy` item so the checks go green.
