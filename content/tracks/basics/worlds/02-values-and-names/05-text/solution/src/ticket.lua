local M = {}

-- "Up", 14 -> "Up, seat 14"
function M.seat_line(film, seat)
  return film .. ", seat " .. seat
end

-- 9 -> "Price: 9 euros"
function M.price_line(price)
  return "Price: " .. price .. " euros"
end

return M
