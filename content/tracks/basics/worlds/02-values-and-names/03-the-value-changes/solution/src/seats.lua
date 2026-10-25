local M = {}

-- the show starts with 100 free seats; a couple books, then a family of four
function M.free_seats()
  local seats = 100
  seats = seats - 2
  seats = seats - 4
  return seats
end

-- the till starts empty; three sales come in: 9, then 18, then 36
function M.till_total()
  local till = 0
  till = till + 9
  till = till + 18
  till = till + 36
  return till
end

return M
