-- Kit Mod: a row of its own in the crafting menu, and the alloy.
data:extend({
  { type = "item-subgroup", name = "kit-parts", group = "intermediate-products", order = "z" },
  {
    type = "item",
    name = "kit-alloy",
    icon = "__base__/graphics/icons/iron-plate.png",
    stack_size = 100,
    subgroup = "intermediate-product",
    order = "z[kit-alloy]",
  },
})

-- TODO: this loop runs before kit-extras has added its items. Move it to data-updates.lua.
-- Move every item whose name starts with "kit-" into the kit-parts row.
-- string.sub(name, 1, 4) is the first four letters of name.
for name, item in pairs(data.raw.item) do
  if string.sub(name, 1, 4) == "kit-" then
    item.subgroup = "kit-parts"
  end
end
