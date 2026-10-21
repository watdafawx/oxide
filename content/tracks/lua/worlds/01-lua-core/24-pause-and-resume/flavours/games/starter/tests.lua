local t = require("oxide.test")
local feed = require("recipe_feed")

t.test("for_loop_gets_each_scaled_count", function()
  local got = {}
  for value in feed.scaled({2, 1, 3}, 5) do
    got[#got + 1] = value
  end
  t.deep_eq(got, {10, 5, 15})
end)

t.test("pauses_between_calls", function()
  local next_value = feed.scaled({4, 5}, 2)
  t.eq(next_value(), 8, "first call: the first count")
  t.eq(next_value(), 10, "second call picks up where it paused")
  t.is_nil(next_value(), "nothing left")
end)
