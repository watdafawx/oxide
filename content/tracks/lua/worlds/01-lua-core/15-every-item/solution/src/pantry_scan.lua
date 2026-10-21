local M = {}

-- {eggs = 12, flour = 3} -> 15
function M.total_items(pantry)
  local total = 0
  for _, count in pairs(pantry) do
    total = total + count
  end
  return total
end

-- delete every entry whose count is 0, in place; return the same table
function M.remove_empty(pantry)
  for item, count in pairs(pantry) do
    if count == 0 then
      pantry[item] = nil
    end
  end
  return pantry
end

return M
