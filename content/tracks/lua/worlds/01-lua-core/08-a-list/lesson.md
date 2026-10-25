A recipe is a list: flour, eggs, milk. The calculator needs to keep the whole list under one name and then ask it questions: what comes first, how many are there?

**One value, many items.** `{"flour", "eggs", "milk"}` is a **table**: one value that holds several. Written like this, in a row, it is a **list**.

**Pick one out.** `recipe[1]` is the first item. Lua counts from **1**, not 0, so the position of an item is its **index**. `recipe[0]` does not exist; it is `nil`.

**Count them.** `#recipe` is the number of items. The `#` goes in front of the name.

**The last one.** The count is also the position of the last item, so `recipe[#recipe]` is the last item.

```lua
local recipe = {"flour", "eggs", "milk"}
local first = recipe[1]        -- "flour"
local how_many = #recipe       -- 3
local last = recipe[#recipe]   -- "milk"
```

**If this is new.** Programming 101 explains lists and indexes from zero: [A list](/quest/basics/06-collections/01-a-list).

## Your turn

- In `first_ingredient`, return the item at index 1.
- In `ingredient_count`, return how many items the list has.
- In `last_ingredient`, return the item at the last index.
