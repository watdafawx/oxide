local M = {}

-- a generator for `for v in range(1, 3) do`: yields a, a+1, ... b
function M.range(a, b)
  -- TODO: return coroutine.wrap(function() ... end) whose body loops from a to b and calls coroutine.yield(i) for each i
  return nil
end

-- a coroutine: resume(co, x) adds x to a running total and yields the total
function M.accumulator()
  -- TODO: coroutine.create(function(x) ... end): keep total = 0 and loop forever: add x, then x = coroutine.yield(total)
  return nil
end

-- tasks: list of functions that call coroutine.yield() between steps.
-- Give every task one step per round until all are finished.
function M.round_robin(tasks)
  -- TODO: coroutine.create each task into a list; then keep looping over the list, coroutine.resume each one whose coroutine.status is not "dead", until every one is dead
end

return M
