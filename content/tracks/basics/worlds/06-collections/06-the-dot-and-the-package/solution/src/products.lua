local M = {}

-- add a new product, or change the price of one that exists
function M.set_price(prices, item, price)
  prices[item] = price
end

-- the product is no longer sold
function M.discontinue(prices, item)
  prices[item] = nil
end

return M
