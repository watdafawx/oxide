local t = require("oxide.test")
local c = require("closures")

t.test("counter_counts_independently", function()
  local a, b = c.make_counter(), c.make_counter()
  t.eq(a(), 1)
  t.eq(a(), 2)
  t.eq(b(), 1, "b has its own count")
  t.eq(a(), 3)
end)

t.test("loop_closures_capture_each_value", function()
  local fns = c.make_multipliers(3)
  t.eq(#fns, 3)
  t.eq(fns[1](10), 10)
  t.eq(fns[2](10), 20)
  t.eq(fns[3](10), 30)
end)

t.test("memoize_calls_once", function()
  local calls = 0
  local slow = function(x)
    calls = calls + 1
    return x * x
  end
  local fast = c.memoize(slow)
  t.eq(fast(4), 16)
  t.eq(fast(4), 16)
  t.eq(calls, 1, "second call came from the cache")
  t.eq(fast(5), 25)
  t.eq(calls, 2, "a new argument runs the function")
end)
