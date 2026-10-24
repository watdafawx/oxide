local M = {}

-- 3 tickets and 2 popcorns; a ticket costs 9, a popcorn 4
function M.order_total()
  local price = 0   -- TODO: a ticket costs 9
  local popcorn = 0 -- TODO: a popcorn costs 4
  return 0          -- TODO: 3 tickets and 2 popcorns, using the two names
end

-- 2 tickets and 1 popcorn at the late show, where tickets are cheaper
function M.late_show_total()
  local price = 9 -- TODO: late-show tickets cost 7
  local popcorn = 4
  return price * 2 + popcorn
end

return M
