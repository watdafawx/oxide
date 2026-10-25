local M = {}

-- 4 friends: a ticket each at 9, and one popcorn at 4 to share
function M.group_total()
  return 4 * 9 + 4
end

-- the friends split that bill evenly
function M.price_each()
  return (4 * 9 + 4) / 4
end

return M
