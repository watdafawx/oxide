local M = {}

-- a helper, already done: the plan for each day of the month, in km
-- (day 1: 3.5 km, day 2: 4 km ... each day half a km more)
local function planned_km(day)
  return 3 + day * 0.5
end

-- the first day (1 to 31) whose planned run is longer than km, or nil
function M.first_day_over(km)
  -- TODO: loop over the days and stop at the first one past km;
  -- give back nil only when no day gets there
  return 0
end

return M
