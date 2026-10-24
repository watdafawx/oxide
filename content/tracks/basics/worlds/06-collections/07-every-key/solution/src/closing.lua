local M = {}

-- {milk = 3, eggs = 12} -> 15
function M.items_in_stock(stock)
  local total = 0
  for item, count in pairs(stock) do
    total = total + count
  end
  return total
end

-- {milk = 3}, {milk = 1.2} -> 3.6: count times price, for every product
function M.stock_value(stock, prices)
  local total = 0
  for item, count in pairs(stock) do
    total = total + count * prices[item]
  end
  return total
end

return M
