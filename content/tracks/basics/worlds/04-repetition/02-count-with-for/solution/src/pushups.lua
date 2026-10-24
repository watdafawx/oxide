local M = {}

-- day 1: 1 push-up, day 2: 2, day 3: 3 ... how many in total after `days` days?
function M.pushups_after(days)
  local total = 0
  for day = 1, days do
    total = total + day
  end
  return total
end

return M
