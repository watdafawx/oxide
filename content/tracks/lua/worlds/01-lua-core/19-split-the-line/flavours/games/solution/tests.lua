local t = require("oxide.test")
local lines = require("recipe_lines")

t.test("splits_on_commas", function()
  t.deep_eq(lines.split_commas("2 iron-plate,1 copper-cable"), {"2 iron-plate", "1 copper-cable"})
  t.deep_eq(lines.split_commas("stone"), {"stone"})
  t.deep_eq(lines.split_commas(""), {})
end)

t.test("parses_a_whole_recipe", function()
  t.deep_eq(lines.parse_recipe("1 iron-plate,3 copper-cable"),
    {{count = 1, name = "iron-plate"}, {count = 3, name = "copper-cable"}})
  t.deep_eq(lines.parse_recipe(""), {})
end)
