local M = {}

-- a generator for `for v in range(1, 3) do`: yields a, a+1, ... b
function M.range(a, b)
  return coroutine.wrap(function()
    for i = a, b do
      coroutine.yield(i)
    end
  end)
end

-- a coroutine: resume(co, x) adds x to a running total and yields the total
function M.accumulator()
  return coroutine.create(function(x)
    local total = 0
    while true do
      total = total + x
      x = coroutine.yield(total)
    end
  end)
end

-- tasks: list of functions that call coroutine.yield() between steps.
-- Give every task one step per round until all are finished.
function M.round_robin(tasks)
  local running = {}
  for i, task in ipairs(tasks) do
    running[i] = coroutine.create(task)
  end
  while #running > 0 do
    local i = 1
    while i <= #running do
      local ok, err = coroutine.resume(running[i])
      if not ok then
        error(err, 0)
      end
      if coroutine.status(running[i]) == "dead" then
        table.remove(running, i)
      else
        i = i + 1
      end
    end
  end
end

return M
