local M = {}

-- start at 2 km, add 1.5 km every week; how many weeks until km reaches goal?
function M.weeks_until(goal)
  local km = 2
  local weeks = 0
  while km < goal do
    km = km + 1.5
    weeks = weeks + 1
  end
  return weeks
end

return M
