local M = {}

-- ({eggs = 12}, "eggs") -> 12; ({}, "eggs") -> 0
function M.count_of(pantry, item)
  -- TODO: pantry[item], or 0 when it is missing
  return nil
end

-- add n of item; the key may not exist yet
function M.put_in(pantry, item, n)
  -- TODO: pantry[item] = what is there (or 0) plus n
end

-- remove item from the pantry and return how many there were (0 if none)
function M.empty_out(pantry, item)
  -- TODO: remember the count, set the key to nil, return the count
  return nil
end

return M
