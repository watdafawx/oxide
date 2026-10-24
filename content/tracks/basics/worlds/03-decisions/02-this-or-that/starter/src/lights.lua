local M = {}

-- "green" for riders of 120 cm and taller, "red" for everyone else
function M.light_colour(height)
  return nil -- TODO: two branches, one colour each
end

-- the ride photo: 8 with a printed copy, 3 for just the download
function M.photo_price(wants_print)
  return nil -- TODO: two branches, one price each
end

return M
