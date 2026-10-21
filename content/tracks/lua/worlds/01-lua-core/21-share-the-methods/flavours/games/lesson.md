In the last quest every recipe built its own copy of every method. With 200 recipes that is 200 copies of the same function. Lua can keep one copy and let every recipe use it. Factorio's own objects work this way.

**One place for methods.** Put the methods in a single table, say `Recipe`, written once. Objects then only hold their own data, like a name and counts.

**Missing key, look elsewhere.** A table can have a **metatable**: a second table of rules about it. If the rules contain an `__index` field, Lua looks there whenever a key is missing from the object.

**Wire it up.** `setmetatable(object, Recipe)` makes `Recipe` the rules for `object`, and `Recipe.__index = Recipe` says "for a missing key, look in `Recipe` itself".

```lua
local Recipe = {}
Recipe.__index = Recipe              -- missing keys are looked up in Recipe

function Recipe:ingredient_count()
  return #self.counts
end

function Recipe.new(name, counts)
  return setmetatable({name = name, counts = counts}, Recipe)
end
```

**One function, many objects.** `recipe:ingredient_count()` finds no such key on the object, looks in `Recipe`, and runs the shared function with `self` set to the object.

## Your turn

- Under `local Chest = {}`, set `Chest.__index` to `Chest`.
- In `Chest.new`, return `setmetatable` with a table holding `items`, and `Chest` as the rules.
- In `Chest:add`, add `n` to that item's count in `self.items`.
