local M = {}

-- ({coal = 12}, "coal") -> 12; ({}, "coal") -> 0
function M.count_of(chest, item)
  -- TODO: chest[item], or 0 when it is missing
  return nil
end

-- add n of item; the key may not exist yet
function M.put_in(chest, item, n)
  -- TODO: chest[item] = what is there (or 0) plus n
end

-- remove item from the chest and return how many there were (0 if none)
function M.empty_out(chest, item)
  -- TODO: remember the count, set the key to nil, return the count
  return nil
end

return M
