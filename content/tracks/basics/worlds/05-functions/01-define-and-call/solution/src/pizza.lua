local M = {}

-- the price of a plain pizza
function M.base_price()
  return 7
end

-- the price of one topping
function M.topping_price()
  return 1.5
end

return M
