The Kit Mod wants a sturdier gear, `kit-gear`: vanilla's iron gear wheel, but stacking to 200 and costing 4 plates instead of 2. Writing it out by hand repeats vanilla, so modders copy the vanilla prototype and change what differs.

**Two names, one table.** `local item = data.raw.item["iron-gear-wheel"]` does not copy anything: it is a second name for vanilla's own table, as in [Two names, one table](/quest/lua/01-lua-core/16-two-names-one-table). Change `item` and you change vanilla's gear, for every other mod too.

**Copy inside copy.** A recipe holds tables inside tables: `ingredients` is a list of entry tables. A shallow copy with a `pairs` loop would still share them, so editing an amount would edit vanilla's recipe. You need a deep copy, like the one you wrote in [Copy inside a copy](/quest/lua/01-lua-core/17-copy-inside-copy).

**Factorio's own copy.** Factorio hands every mod a deep copy function as a global, `table.deepcopy(t)`. It copies the table and every table inside it, so the copy shares nothing with the original.

```lua
local item = table.deepcopy(data.raw.item["copper-cable"])
item.name = "kit-wire"
item.stack_size = 400
data:extend({ item })
```

**Always rename.** Extending a prototype whose type and name already exist **silently overwrites** it, in the game and in Oxide. A copy keeps the original's name until you change it, so `name` is the first thing to set.

## Your turn

- Make the first line a deep copy of vanilla's gear item.
- Make the second line a deep copy of vanilla's gear recipe.
