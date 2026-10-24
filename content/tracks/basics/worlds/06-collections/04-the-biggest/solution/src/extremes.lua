local M = {}

-- {2.5, 1, 4} -> 4
function M.most_expensive(basket)
  local highest = basket[1]
  for _, price in ipairs(basket) do
    if price > highest then
      highest = price
    end
  end
  return highest
end

-- {2.5, 1, 4} -> 1; {} -> nil
function M.cheapest(basket)
  local lowest = basket[1]
  for _, price in ipairs(basket) do
    if price < lowest then
      lowest = price
    end
  end
  return lowest
end

return M
