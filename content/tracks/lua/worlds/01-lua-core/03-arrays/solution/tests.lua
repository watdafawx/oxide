local t = require("oxide.test")
local a = require("arrays")

t.test("sum_one_based", function()
  t.eq(a.sum({1, 2, 3}), 6)
  t.eq(a.sum({}), 0)
end)

t.test("append_and_pop", function()
  local xs = {}
  a.append(xs, 5)
  a.append(xs, 7)
  t.deep_eq(xs, {5, 7}, "after two appends")
  t.eq(a.pop(xs), 7, "pop returns the last")
  t.deep_eq(xs, {5}, "and removes it")
end)

t.test("sort_descending", function()
  local xs = {3, 1, 2}
  a.sort_desc(xs)
  t.deep_eq(xs, {3, 2, 1})
end)

t.test("join_with_commas", function()
  t.eq(a.join({"a", "b", "c"}), "a, b, c")
  t.eq(a.join({}), "")
end)
