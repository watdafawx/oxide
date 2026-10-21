local Chest = {}
-- TODO: make a missing key fall back to Chest (one line, the lesson shows it)

-- how many of item this chest holds
function Chest:count_of(item)
  return self.items[item] or 0
end

-- a chest holding items, e.g. {coal = 12}
function Chest.new(items)
  -- TODO: return setmetatable({items = items}, Chest)
  return nil
end

-- add n of item to this chest
function Chest:add(item, n)
  -- TODO: self.items[item] = what is there (or 0) plus n
end

return Chest
