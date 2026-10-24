-- Kit Mod: kit-gear = a sturdier iron gear wheel.
local item = data.raw.item["iron-gear-wheel"]
local recipe = data.raw.recipe["iron-gear-wheel"]

-- TODO: these two lines point at vanilla's own tables, so the edits below change vanilla.
--       Make each one a deep copy, so the edits hit copies instead.
item.name = "kit-gear"
item.stack_size = 200
recipe.name = "kit-gear"
recipe.ingredients[1].amount = 4
recipe.results[1].name = "kit-gear"

data:extend({ item, recipe })
