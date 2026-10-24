local M = {}

-- scaled({1, 2, 3}, 10) hands out 10, then 20, then 30, one per call
function M.scaled(counts, factor)
  -- TODO: return a function made with coroutine.wrap; inside it, loop over counts and
  -- hand out each count times factor with a yield
  return nil
end

return M
