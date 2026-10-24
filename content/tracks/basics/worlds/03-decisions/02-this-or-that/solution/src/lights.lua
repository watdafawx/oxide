local M = {}

-- "green" for riders of 120 cm and taller, "red" for everyone else
function M.light_colour(height)
  if height >= 120 then
    return "green"
  else
    return "red"
  end
end

-- the ride photo: 8 with a printed copy, 3 for just the download
function M.photo_price(wants_print)
  if wants_print then
    return 8
  else
    return 3
  end
end

return M
