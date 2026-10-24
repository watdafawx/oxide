local M = {}

-- a helper, already done: what you ran on each day of the week; -1 means you were ill
-- day:        1   2  3  4   5  6  7
local week = {-1, -1, 6, 4, -1, 8, 6}
local function logged_km(day)
  return week[day]
end

-- the average distance over days 1 to `days`, leaving out the ill days
function M.average_km(days)
  local total = 0
  local count = 0
  for day = 1, days do
    local km = logged_km(day)
    if km ~= -1 then
      total = total + km
      count = count + 1
    end
  end
  if count == 0 then
    return 0
  end
  return total / count
end

return M
