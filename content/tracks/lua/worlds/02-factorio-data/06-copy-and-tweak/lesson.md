**Why now:** writing every prototype by hand gets old. Modders clone a vanilla one and change what differs: a sturdier **kit-gear** that stacks to 200 and costs 4 plates instead of 2.

In Lua `local item = data.raw.item["iron-gear-wheel"]` copies a **reference**, not the table. Edit `item` and you edit vanilla for every other mod too. `table.deepcopy` (a global in Factorio, also `util.table.deepcopy`) copies the table and every table inside it:

```lua
local item = table.deepcopy(data.raw.item["iron-gear-wheel"])
item.name = "kit-gear"
item.stack_size = 200

local recipe = table.deepcopy(data.raw.recipe["iron-gear-wheel"])
recipe.name = "kit-gear"
recipe.ingredients[1].amount = 4     -- nested table: safe, because it was copied too
recipe.results[1].name = "kit-gear"

data:extend({ item, recipe })
```

- A shallow copy (`for k, v in pairs(t) do c[k] = v end`) still shares nested tables such as `ingredients`. Only `deepcopy` is safe for prototypes.
- Always change `name`. Extending a prototype whose type and name already exist **silently overwrites** it, in the game and in Oxide.
- Recipe `results[1].name` and the item `name` must match, or the recipe produces something that does not exist.

## Your turn

- Open `mod/data.lua`. It edits the vanilla gear in place, which is the bug.
- Copy the item and the recipe first, edit the copies, extend with the copies.
