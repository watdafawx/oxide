local M = {}

-- {coal = 12, stone = 3} -> 15
function M.total_items(chest)
  -- TODO: loop with pairs and add up every count
  return nil
end

-- delete every entry whose count is 0, in place; return the same table
function M.remove_empty(chest)
  -- TODO: loop with pairs; set chest[item] = nil when count == 0
  return nil
end

return M
