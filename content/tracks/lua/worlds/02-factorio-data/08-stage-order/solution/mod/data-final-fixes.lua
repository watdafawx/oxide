-- Every mod's data-updates.lua has run by now: the last chance to touch all items.
for _, item in pairs(data.raw.item) do
  if item.stack_size > 50 then
    item.stack_size = math.floor(item.stack_size / 2)
  end
end
