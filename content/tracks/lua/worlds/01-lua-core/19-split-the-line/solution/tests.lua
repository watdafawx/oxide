local t = require("oxide.test")
local lines = require("recipe_lines")

t.test("splits_on_commas", function()
  t.deep_eq(lines.split_commas("3 eggs,2 flour"), {"3 eggs", "2 flour"})
  t.deep_eq(lines.split_commas("salt"), {"salt"})
  t.deep_eq(lines.split_commas(""), {})
end)

t.test("parses_a_whole_recipe", function()
  t.deep_eq(lines.parse_recipe("3 eggs,2 flour,1 brown-sugar"),
    {{count = 3, name = "eggs"}, {count = 2, name = "flour"}, {count = 1, name = "brown-sugar"}})
  t.deep_eq(lines.parse_recipe(""), {})
end)
