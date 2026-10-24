-- Kit Mod, step 4: kit-gear = a sturdier iron gear wheel.
local item = data.raw.item["iron-gear-wheel"]
local recipe = data.raw.recipe["iron-gear-wheel"]

-- TODO: these two lines point at the vanilla tables themselves. Make each one a deep copy
--       instead, so the edits below hit copies, not vanilla.
item.name = "kit-gear"
item.stack_size = 200
recipe.name = "kit-gear"
recipe.ingredients[1].amount = 4
recipe.results[1].name = "kit-gear"

data:extend({ item, recipe })
