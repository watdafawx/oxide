local M = {}

-- {milk = 1.2, bread = 2}, "milk" -> 1.2
function M.price_of(prices, item)
  return prices[item]
end

-- {milk = 1.2}, "milk" -> true; "caviar" -> false
function M.sells(prices, item)
  return prices[item] ~= nil
end

return M
