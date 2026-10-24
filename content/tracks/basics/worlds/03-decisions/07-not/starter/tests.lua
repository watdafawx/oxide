local t = require("oxide.test")
local status = require("status")

t.test("open_when_not_full", function()
  t.eq(status.can_enter(false), true, "not full: come in")
  t.eq(status.can_enter(true), false, "full: wait outside")
end)

t.test("closed_unless_staff_and_weather_are_ok", function()
  t.eq(status.ride_closed(true, true), false, "everything ok: open")
  t.eq(status.ride_closed(false, true), true, "no staff")
  t.eq(status.ride_closed(true, false), true, "bad weather")
  t.eq(status.ride_closed(false, false), true, "neither")
end)
