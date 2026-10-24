local t = require("oxide.test")
local cinema = require("cinema")

t.test("order_costs_35", function()
  t.eq(cinema.order_total(), 35, "3 tickets at 9 and 2 popcorns at 4")
end)

t.test("late_show_order_costs_18", function()
  t.eq(cinema.late_show_total(), 18, "2 tickets at 7 and 1 popcorn at 4")
end)
