local M = {}

-- 7 eggs -> 3.5 (an omelette takes 2 eggs)
function M.omelettes_exact(eggs)
  return eggs / 2
end

-- 7 eggs -> 3 whole omelettes
function M.whole_omelettes(eggs)
  return math.floor(eggs / 2)
end

return M
