local M = {}

-- scaled({1, 2, 3}, 10) hands out 10, then 20, then 30, one per call
function M.scaled(counts, factor)
  -- TODO: return coroutine.wrap(function() ... end) that yields count * factor for each count
  return nil
end

return M
