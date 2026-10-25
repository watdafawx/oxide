local M = {}

-- a helper, already done: the plan for each day of the month, in km
-- (day 1: 3.5 km, day 2: 4 km ... each day half a km more)
local function planned_km(day)
  return 3 + day * 0.5
end

-- the first day (1 to 31) whose planned run is longer than km, or nil
function M.first_day_over(km)
  local found = nil
  for day = 1, 31 do
    if planned_km(day) > km then
      found = day
      break
    end
  end
  return found
end

return M
