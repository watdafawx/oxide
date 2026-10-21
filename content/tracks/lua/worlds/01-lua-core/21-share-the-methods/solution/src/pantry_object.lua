local Pantry = {}
Pantry.__index = Pantry

-- how many of item this pantry holds
function Pantry:count_of(item)
  return self.items[item] or 0
end

-- a pantry holding items, e.g. {eggs = 12}
function Pantry.new(items)
  return setmetatable({items = items}, Pantry)
end

-- add n of item to this pantry
function Pantry:add(item, n)
  self.items[item] = (self.items[item] or 0) + n
end

return Pantry
