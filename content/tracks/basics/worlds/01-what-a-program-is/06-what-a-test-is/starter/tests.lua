local t = require("oxide.test")
local sign = require("sign")

-- t.eq(what your function gave back, what it should be)

t.test("gives_the_opening_hours", function()
  t.eq(sign.opening_hours(), "Open 9 to 5")
end)

t.test("gives_the_closed_day", function()
  t.eq(sign.closed_on(), "Closed on Sundays")
end)
