local t = require("oxide.test")
local tools = require("stack_tools")

t.test("sorts_biggest_first", function()
  local stacks = {30, 90, 40, 10}
  tools.sort_biggest_first(stacks)
  t.deep_eq(stacks, {90, 40, 30, 10})
end)

t.test("applies_a_rule_to_each", function()
  local original = {1, 2, 3}
  local doubled = tools.apply_to_each(original, function(n) return n * 2 end)
  t.deep_eq(doubled, {2, 4, 6})
  t.deep_eq(original, {1, 2, 3}, "the original list is untouched")
  t.deep_eq(tools.apply_to_each({}, function(n) return n end), {})
end)
