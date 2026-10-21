local M = {}

-- ({eggs = 12}, "eggs") -> 12; ({}, "eggs") -> 0
function M.count_of(pantry, item)
  return pantry[item] or 0
end

-- add n of item; the key may not exist yet
function M.put_in(pantry, item, n)
  pantry[item] = (pantry[item] or 0) + n
end

-- remove item from the pantry and return how many there were (0 if none)
function M.empty_out(pantry, item)
  local had = pantry[item] or 0
  pantry[item] = nil
  return had
end

return M
