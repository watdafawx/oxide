local M = {}

-- staff tickets: the normal price, minus the staff discount
function M.staff_price()
  local price = 9
  price = price - 2   -- staff discount
  -- price = price + 2
  return price
end

-- afternoon shows are cheaper
function M.matinee_price()
  local price = 9
  price = 6
  return price
end

return M
