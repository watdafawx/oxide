local M = {}

-- {milk = 1.2, bread = 2}, "milk" -> 1.2
function M.price_of(prices, item)
  return nil -- TODO: look item up
end

-- {milk = 1.2}, "milk" -> true; "caviar" -> false
function M.sells(prices, item)
  return nil -- TODO: is there a price under item?
end

return M
