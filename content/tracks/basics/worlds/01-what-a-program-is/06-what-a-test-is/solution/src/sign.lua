local M = {}

-- the line about opening hours; tests.lua says the exact text
function M.opening_hours()
  return "Open 9 to 5"
end

-- the line about the day the shop is closed
function M.closed_on()
  return "Closed on Sundays"
end

return M
