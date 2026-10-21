local M = {}

-- 7 plates -> 3.5 (a gear takes 2 iron plates)
function M.gears_exact(plates)
  return plates / 2
end

-- 7 plates -> 3 whole gears
function M.whole_gears(plates)
  return math.floor(plates / 2)
end

return M
