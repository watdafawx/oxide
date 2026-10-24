local Chest = {}
-- TODO: make a missing key fall back to Chest (one line, the lesson shows it)

-- how many of item this chest holds
function Chest:count_of(item)
  return self.items[item] or 0
end

-- a chest holding items, e.g. {coal = 12}
function Chest.new(items)
  -- TODO: make a table holding items and set its metatable so lookups fall back to Chest; return it
  return nil
end

-- add n of item to this chest
function Chest:add(item, n)
  -- TODO: store what is there (or 0) plus n back under item
end

return Chest
