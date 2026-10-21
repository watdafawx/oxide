local M = {}

-- a new table with the same keys and counts as pantry
function M.copy_pantry(pantry)
  local copy = {}
  for key, value in pairs(pantry) do
    copy[key] = value
  end
  return copy
end

return M
