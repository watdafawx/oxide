local M = {}

-- how many batches of pancakes you plan to bake
function M.planned_batches()
  local batches = 4
  return batches
end

-- how many eggs those batches need (3 eggs per batch)
function M.eggs_needed()
  local batches = 4
  local eggs = batches * 3
  return eggs
end

return M
