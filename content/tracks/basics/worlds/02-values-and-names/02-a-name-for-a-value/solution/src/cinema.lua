local M = {}

-- 3 tickets and 2 popcorns; a ticket costs 9, a popcorn 4
function M.order_total()
  local price = 9
  local popcorn = 4
  return price * 3 + popcorn * 2
end

-- 2 tickets and 1 popcorn at the late show, where tickets are cheaper
function M.late_show_total()
  local price = 7
  local popcorn = 4
  return price * 2 + popcorn
end

return M
