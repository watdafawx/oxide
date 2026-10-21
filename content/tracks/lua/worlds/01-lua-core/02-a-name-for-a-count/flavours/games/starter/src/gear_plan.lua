local M = {}

-- how many gears you want to make
function M.gears_wanted()
  local gears = 0 -- TODO: you want 5 gears
  return gears
end

-- how many iron plates those gears cost (2 plates per gear)
function M.plates_needed()
  local gears = 5
  local plates = 0 -- TODO: 2 plates per gear, so gears * 2
  return plates
end

return M
