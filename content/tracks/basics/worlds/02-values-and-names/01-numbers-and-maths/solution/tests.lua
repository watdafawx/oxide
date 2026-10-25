local t = require("oxide.test")
local tickets = require("tickets")

t.test("group_total_is_40", function()
  t.eq(tickets.group_total(), 40, "4 tickets at 9 plus one popcorn at 4")
end)

t.test("each_friend_pays_10", function()
  t.eq(tickets.price_each(), 10, "40 split between 4 friends")
end)
