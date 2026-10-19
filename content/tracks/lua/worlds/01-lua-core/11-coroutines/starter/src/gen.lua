local M = {}

-- a generator for `for v in range(1, 3) do`: yields a, a+1, ... b
function M.range(a, b)
  -- TODO: coroutine.wrap + coroutine.yield
  return nil
end

-- a coroutine: resume(co, x) adds x to a running total and yields the total
function M.accumulator()
  -- TODO: coroutine.create; the value you pass to resume comes back from yield
  return nil
end

-- tasks: list of functions that call coroutine.yield() between steps.
-- Give every task one step per round until all are finished.
function M.round_robin(tasks)
  -- TODO: coroutine.create each task, then resume them in turn
end

return M
