A Factorio recipe lists its ingredients. The calculator needs to keep that list under one name and then ask it questions: what comes first, how many are there?

**One value, many items.** `{"iron-plate", "copper-cable"}` is a **table**: one value that holds several. Written like this, in a row, it is a **list**.

**Pick one out.** `recipe[1]` is the first item. Lua counts from **1**, not 0, so the position of an item is its **index**. `recipe[0]` does not exist; it is `nil`.

**Count them.** `#recipe` is the number of items. The `#` goes in front of the name.

**The last one.** The count is also the position of the last item, so `recipe[#recipe]` is the last item.

```lua
local recipe = {"iron-plate", "copper-cable"}
local first = recipe[1]        -- "iron-plate"
local how_many = #recipe       -- 2
local last = recipe[#recipe]   -- "copper-cable"
```

## Your turn

- In `first_ingredient`, return the item at index 1.
- In `ingredient_count`, return how many items the list has.
- In `last_ingredient`, return the item at the last index.
