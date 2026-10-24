local t = require("oxide.test")
local prices = require("prices")

t.test("prices_any_number_of_tickets", function()
  t.eq(prices.ticket_cost(1), 9, "one ticket")
  t.eq(prices.ticket_cost(3), 27, "three tickets")
  t.eq(prices.ticket_cost(10), 90, "a school class of ten")
end)

t.test("prices_tickets_and_popcorn", function()
  t.eq(prices.order_cost(2, 1), 22, "2 tickets, 1 box")
  t.eq(prices.order_cost(3, 0), 27, "no popcorn at all")
  t.eq(prices.order_cost(1, 2), 17, "1 ticket, 2 boxes")
end)
