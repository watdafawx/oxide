local M = {}

-- a helper, already done: what you ran on each day of the week; -1 means you were ill
-- day:        1   2  3  4   5  6  7
local week = {-1, -1, 6, 4, -1, 8, 6}
local function logged_km(day)
  return week[day]
end

-- the average distance over days 1 to `days`, leaving out the ill days
function M.average_km(days)
  -- TODO: a total and a count, a loop that skips -1, and no dividing by 0
  return nil
end

return M
