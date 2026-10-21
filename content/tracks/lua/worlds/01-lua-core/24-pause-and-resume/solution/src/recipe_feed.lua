local M = {}

-- scaled({1, 2, 3}, 10) hands out 10, then 20, then 30, one per call
function M.scaled(counts, factor)
  return coroutine.wrap(function()
    for _, count in ipairs(counts) do
      coroutine.yield(count * factor)
    end
  end)
end

return M
