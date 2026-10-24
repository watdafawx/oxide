local M = {}

-- delivery costs 2 for the first 3 km, then 1 more for every km after that
function M.delivery_fee(km)
  local fee = 2
  if km > 3 then
    fee = 2 + (km - 3)
  end
  return fee
end

-- the food price plus the delivery fee
function M.order_total(food, km)
  return food + M.delivery_fee(km)
end

return M
