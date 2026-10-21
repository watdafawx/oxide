local M = {}

-- a new table with the same keys and counts as chest
function M.copy_chest(chest)
  local copy = {}
  for key, value in pairs(chest) do
    copy[key] = value
  end
  return copy
end

return M
