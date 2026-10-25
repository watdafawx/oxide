local M = {}

-- "Up", 14 -> "Up, seat 14"
function M.seat_line(film, seat)
  return "" -- TODO: glue the film, the words in between and the seat
end

-- 9 -> "Price: 9 euros"
function M.price_line(price)
  return "" -- TODO: text before the price, the price, text after it
end

return M
