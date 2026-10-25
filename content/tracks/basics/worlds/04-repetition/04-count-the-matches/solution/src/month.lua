local M = {}

-- a helper, already done: true on days 7, 14, 21, 28 ...
-- (% is the remainder after dividing: 14 % 7 is 0, 15 % 7 is 1)
local function is_rest_day(day)
  return day % 7 == 0
end

-- how many of the first `days` days are running days (not rest days)?
function M.running_days(days)
  local count = 0
  for day = 1, days do
    if not is_rest_day(day) then
      count = count + 1
    end
  end
  return count
end

return M
