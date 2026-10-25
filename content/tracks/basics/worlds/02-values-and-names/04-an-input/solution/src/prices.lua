local M = {}

-- the price of any number of tickets, at 9 each
function M.ticket_cost(tickets)
  return tickets * 9
end

-- tickets at 9 each, plus popcorn boxes at 4 each
function M.order_cost(tickets, boxes)
  return tickets * 9 + boxes * 4
end

return M
