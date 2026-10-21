local Pantry = {}
-- TODO: make a missing key fall back to Pantry (one line, the lesson shows it)

-- how many of item this pantry holds
function Pantry:count_of(item)
  return self.items[item] or 0
end

-- a pantry holding items, e.g. {eggs = 12}
function Pantry.new(items)
  -- TODO: return setmetatable({items = items}, Pantry)
  return nil
end

-- add n of item to this pantry
function Pantry:add(item, n)
  -- TODO: self.items[item] = what is there (or 0) plus n
end

return Pantry
