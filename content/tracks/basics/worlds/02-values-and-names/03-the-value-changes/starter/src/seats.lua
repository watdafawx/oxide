local M = {}

-- the show starts with 100 free seats; a couple books, then a family of four
function M.free_seats()
  local seats = 100
  -- TODO: take away the couple's seats, then the family's, one line each
  return seats
end

-- the till starts empty; three sales come in: 9, then 18, then 36
function M.till_total()
  local till = 0
  -- TODO: add each sale to till, one line each
  return till
end

return M
