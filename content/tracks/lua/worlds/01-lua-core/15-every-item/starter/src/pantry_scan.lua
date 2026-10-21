local M = {}

-- {eggs = 12, flour = 3} -> 15
function M.total_items(pantry)
  -- TODO: loop with pairs and add up every count
  return nil
end

-- delete every entry whose count is 0, in place; return the same table
function M.remove_empty(pantry)
  -- TODO: loop with pairs; set pantry[item] = nil when count == 0
  return nil
end

return M
