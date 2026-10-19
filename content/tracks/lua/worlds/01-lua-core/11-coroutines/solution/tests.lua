local t = require("oxide.test")
local g = require("gen")

t.test("generator_yields_in_order", function()
  local seen = {}
  for v in g.range(1, 3) do seen[#seen + 1] = v end
  t.deep_eq(seen, {1, 2, 3})
  local none = {}
  for v in g.range(5, 4) do none[#none + 1] = v end
  t.deep_eq(none, {}, "empty when a > b")
end)

t.test("passes_values_both_ways", function()
  local co = g.accumulator()
  t.eq(coroutine.status(co), "suspended", "a new coroutine waits for the first resume")
  local ok, total = coroutine.resume(co, 5)
  t.truthy(ok)
  t.eq(total, 5)
  ok, total = coroutine.resume(co, 10)
  t.eq(total, 15)
  ok, total = coroutine.resume(co, -3)
  t.eq(total, 12)
end)

t.test("round_robin_runs_tasks", function()
  local log = {}
  local function task(name, steps)
    return function()
      for i = 1, steps do
        log[#log + 1] = name .. i
        coroutine.yield()
      end
    end
  end
  g.round_robin({task("a", 2), task("b", 1), task("c", 3)})
  t.deep_eq(log, {"a1", "b1", "c1", "a2", "c2", "c3"})
end)
