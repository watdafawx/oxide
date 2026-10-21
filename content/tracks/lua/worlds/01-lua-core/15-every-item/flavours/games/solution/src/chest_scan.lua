local M = {}

-- {coal = 12, stone = 3} -> 15
function M.total_items(chest)
  local total = 0
  for _, count in pairs(chest) do
    total = total + count
  end
  return total
end

-- delete every entry whose count is 0, in place; return the same table
function M.remove_empty(chest)
  for item, count in pairs(chest) do
    if count == 0 then
      chest[item] = nil
    end
  end
  return chest
end

return M
