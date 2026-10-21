local M = {}

-- ({coal = 12}, "coal") -> 12; ({}, "coal") -> 0
function M.count_of(chest, item)
  return chest[item] or 0
end

-- add n of item; the key may not exist yet
function M.put_in(chest, item, n)
  chest[item] = (chest[item] or 0) + n
end

-- remove item from the chest and return how many there were (0 if none)
function M.empty_out(chest, item)
  local had = chest[item] or 0
  chest[item] = nil
  return had
end

return M
