local t = require("oxide.test")
local header = require("header")

t.test("underlines_with_equals_signs", function()
  t.eq(header.underline(5), "=====")
  t.eq(header.underline(1), "=")
  t.eq(header.underline(0), "", "zero signs is empty text")
end)

t.test("team_names_in_capitals", function()
  t.eq(header.banner("owls"), "OWLS")
  t.eq(header.banner("Red Foxes"), "RED FOXES")
end)

t.test("score_stars_with_spaces", function()
  t.eq(header.stars(3), "* * *")
  t.eq(header.stars(1), "*", "one star needs no space")
end)
