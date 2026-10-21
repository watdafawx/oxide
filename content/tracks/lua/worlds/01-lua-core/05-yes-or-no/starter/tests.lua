local t = require("oxide.test")
local p = require("pantry_checks")

t.test("three_eggs_make_a_batch", function()
  t.eq(p.can_make_batch(3), true)
  t.eq(p.can_make_batch(12), true)
  t.eq(p.can_make_batch(2), false)
end)

t.test("full_carton_holds_12", function()
  t.eq(p.carton_is_full(12), true)
  t.eq(p.carton_is_full(11), false)
end)

t.test("partial_carton_is_not_12", function()
  t.eq(p.carton_is_partial(5), true)
  t.eq(p.carton_is_partial(12), false)
end)
