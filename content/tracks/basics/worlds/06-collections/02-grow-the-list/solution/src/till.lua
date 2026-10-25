local M = {}

-- a scanned price goes at the end of the basket
function M.scan(basket, price)
  table.insert(basket, price)
end

-- the cashier corrects the price in one slot
function M.fix_price(basket, slot, price)
  basket[slot] = price
end

return M
