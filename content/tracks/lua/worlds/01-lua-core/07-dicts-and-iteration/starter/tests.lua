local t = require("oxide.test")
local d = require("dicts")

t.test("counts_words", function()
  t.deep_eq(d.count_words("iron copper iron coal iron"), {iron = 3, copper = 1, coal = 1})
  t.deep_eq(d.count_words(""), {})
end)

t.test("removes_during_pairs_safely", function()
  local stock = {iron = 12, copper = 0, coal = 0, gold = 1}
  local same = d.remove_empty(stock)
  t.deep_eq(stock, {iron = 12, gold = 1}, "zeros are gone")
  t.truthy(same == stock, "same table, edited in place")
end)

t.test("deep_copy_is_independent", function()
  local recipe = {name = "gear", ingredients = {{"iron-plate", 2}}, time = 0.5}
  local copy = d.deep_copy(recipe)
  t.deep_eq(copy, recipe, "same contents")
  t.falsy(copy == recipe, "different table")
  t.falsy(copy.ingredients == recipe.ingredients, "nested tables are copied too")
  copy.ingredients[1][2] = 99
  t.eq(recipe.ingredients[1][2], 2, "original untouched")
end)
