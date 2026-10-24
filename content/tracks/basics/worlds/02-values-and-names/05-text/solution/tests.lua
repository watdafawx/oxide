local t = require("oxide.test")
local ticket = require("ticket")

t.test("builds_the_seat_line", function()
  t.eq(ticket.seat_line("Up", 14), "Up, seat 14")
  t.eq(ticket.seat_line("Jaws", 3), "Jaws, seat 3")
end)

t.test("builds_the_price_line", function()
  t.eq(ticket.price_line(9), "Price: 9 euros")
  t.eq(ticket.price_line(7.5), "Price: 7.5 euros")
end)
