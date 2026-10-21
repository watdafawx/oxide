local t = require("oxide.test")
local s = require("stack_checks")

t.test("two_plates_make_a_gear", function()
  t.eq(s.can_craft_gear(2), true)
  t.eq(s.can_craft_gear(50), true)
  t.eq(s.can_craft_gear(1), false)
end)

t.test("full_stack_holds_100", function()
  t.eq(s.stack_is_full(100), true)
  t.eq(s.stack_is_full(99), false)
end)

t.test("partial_stack_is_not_100", function()
  t.eq(s.stack_is_partial(40), true)
  t.eq(s.stack_is_partial(100), false)
end)
