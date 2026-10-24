local M = {}

-- {2.5, 1, 4} -> 7.5
function M.basket_total(basket)
  local total = 0
  for _, price in ipairs(basket) do
    total = total + price
  end
  return total
end

-- {2.5, 1, 4}, limit 2 -> 2 (2.5 and 4 are above 2)
function M.count_over(basket, limit)
  local count = 0
  for _, price in ipairs(basket) do
    if price > limit then
      count = count + 1
    end
  end
  return count
end

return M
