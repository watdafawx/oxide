local Chest = {}
Chest.__index = Chest

-- how many of item this chest holds
function Chest:count_of(item)
  return self.items[item] or 0
end

-- a chest holding items, e.g. {coal = 12}
function Chest.new(items)
  return setmetatable({items = items}, Chest)
end

-- add n of item to this chest
function Chest:add(item, n)
  self.items[item] = (self.items[item] or 0) + n
end

return Chest
