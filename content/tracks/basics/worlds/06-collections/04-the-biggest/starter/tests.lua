local t = require("oxide.test")
local extremes = require("extremes")

t.test("finds_the_most_expensive", function()
  t.eq(extremes.most_expensive({2.5, 1, 4}), 4)
  t.eq(extremes.most_expensive({9, 3, 7}), 9, "the first one is the dearest")
  t.eq(extremes.most_expensive({0.5}), 0.5, "a single item")
end)

t.test("finds_the_cheapest", function()
  t.eq(extremes.cheapest({2.5, 1, 4}), 1)
  t.eq(extremes.cheapest({3, 0.5, 2}), 0.5, "0.5 is in the basket; 0 is not")
  t.is_nil(extremes.cheapest({}), "an empty basket has no cheapest item")
end)
