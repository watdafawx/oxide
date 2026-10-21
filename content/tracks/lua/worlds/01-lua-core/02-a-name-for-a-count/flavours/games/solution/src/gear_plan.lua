local M = {}

-- how many gears you want to make
function M.gears_wanted()
  local gears = 5
  return gears
end

-- how many iron plates those gears cost (2 plates per gear)
function M.plates_needed()
  local gears = 5
  local plates = gears * 2
  return plates
end

return M
