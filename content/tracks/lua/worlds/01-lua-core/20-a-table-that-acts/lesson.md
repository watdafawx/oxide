A recipe knows its name and its counts. It would be neater if it could also answer questions about itself, like "how many items in total?", so the calculator can ask the recipe instead of poking inside it.

**A function in a table.** A function is a value, so a table can hold one under a key, like any other value. You can write the definition as `function recipe.total(self)`.

**Hand it itself.** `recipe.total(recipe)` passes the recipe in as the first parameter. By habit that parameter is called `self`, and inside the function `self.counts` is the recipe's own counts.

**The shortcut.** A colon does both jobs. `recipe:total()` means `recipe.total(recipe)`, and `function recipe:total()` gives the function a `self` for free.

```lua
local recipe = {name = "pancakes", counts = {3, 1, 2}}

function recipe:ingredient_count()
  return #self.counts
end

local n = recipe:ingredient_count()   -- 3
```

**Methods call methods.** A function written this way is a **method**. Inside one, `self:other()` calls a sibling method on the same table.

## Your turn

- In `total_items`, add up `self.counts` with a loop.
- In `describe`, glue the name, `": "`, the total and `" items"` together.
- Save. Both objectives turn green.
