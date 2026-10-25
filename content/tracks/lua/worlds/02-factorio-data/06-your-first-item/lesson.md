The Kit Mod's first new thing is `kit-alloy`, a plate made from iron and copper. In Factorio, a new item is one more prototype in `data.raw.item`, and your `data.lua` adds it.

**Add a prototype.** `data:extend` is a method on the global `data` table, called with a colon like the methods in the last world. It puts each prototype you hand it into `data.raw`, under its type and its name.

**Every prototype.** Each one needs a `type`, which says what kind of thing it is (`"item"`) and so which part of `data.raw` it lands in, and a `name` no other prototype of that type uses.

**A list, even for one.** `data:extend` takes a list of prototypes, so even one item sits inside an outer `{ }`: `data:extend({ {...} })`. The docs call a list an **array**. Forget the outer braces and the game stops with "Expected array of prototypes".

**What an item needs.** An item also needs a picture, `icon`, and a `stack_size`: how many fit in one slot, a whole number of at least 1. Until the mod ships its own pictures, borrow vanilla's from `"__base__/graphics/icons/..."`; a later quest explains that path.

```lua
data:extend({
  {
    type = "item",
    name = "kit-bolt",
    icon = "__base__/graphics/icons/copper-plate.png",
    stack_size = 200,
  },
})
```

**Checked at the end.** The game checks the required fields when the data stage ends, not at `data:extend`, because a later line or another mod may still fill one in. Oxide does the same, and its message names the missing field.

## Your turn

- In `data.lua`, give `kit-alloy` the iron plate's icon.
- Give it a stack size of 100.
