local t = require("oxide.test")
local seats = require("seats")

t.test("six_seats_are_taken", function()
  t.eq(seats.free_seats(), 94, "100 seats, minus 2, minus 4")
end)

t.test("the_till_adds_every_sale", function()
  t.eq(seats.till_total(), 63, "0 + 9 + 18 + 36")
end)
