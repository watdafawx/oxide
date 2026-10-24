local t = require("oxide.test")
local compare = require("compare")

t.test("alike_pictures_match", function()
  t.eq(compare.same_colours({"red", "blue"}, {"red", "blue"}), true, "two separate lists, same colours")
  t.eq(compare.same_colours({}, {}), true, "two empty pictures")
end)

t.test("different_pictures_do_not_match", function()
  t.eq(compare.same_colours({"red", "blue"}, {"red", "green"}), false, "the second colour differs")
  t.eq(compare.same_colours({"red"}, {"red", "blue"}), false, "different lengths")
  t.eq(compare.same_colours({"red", "blue"}, {"blue", "red"}), false, "same colours, other order")
end)
