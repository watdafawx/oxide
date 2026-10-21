local t = require("oxide.test")
local tools = require("list_tools")

t.test("sorts_biggest_first", function()
  local counts = {3, 9, 4, 1}
  tools.sort_biggest_first(counts)
  t.deep_eq(counts, {9, 4, 3, 1})
end)

t.test("applies_a_rule_to_each", function()
  local original = {1, 2, 3}
  local doubled = tools.apply_to_each(original, function(n) return n * 2 end)
  t.deep_eq(doubled, {2, 4, 6})
  t.deep_eq(original, {1, 2, 3}, "the original list is untouched")
  t.deep_eq(tools.apply_to_each({}, function(n) return n end), {})
end)
