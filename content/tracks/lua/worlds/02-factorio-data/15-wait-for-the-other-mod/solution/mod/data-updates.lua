-- Every mod's data.lua has run by now, add-ons included.
-- Move every item whose name starts with "kit-" into the kit-parts row.
-- string.sub(name, 1, 4) is the first four letters of name.
for name, item in pairs(data.raw.item) do
  if string.sub(name, 1, 4) == "kit-" then
    item.subgroup = "kit-parts"
  end
end
