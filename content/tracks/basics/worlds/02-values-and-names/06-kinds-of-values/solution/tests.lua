local t = require("oxide.test")
local keypad = require("keypad")

t.test("keypad_text_becomes_a_number", function()
  local n = keypad.from_keypad("3")
  t.is_type(n, "number", "the text should become a number")
  t.eq(n, 3)
  t.is_nil(keypad.from_keypad("two"), '"two" is not a number')
end)

t.test("prices_what_was_typed", function()
  t.eq(keypad.cost_of_typed("4"), 36, "four tickets at 9")
  t.eq(keypad.cost_of_typed("1"), 9, "one ticket")
end)
