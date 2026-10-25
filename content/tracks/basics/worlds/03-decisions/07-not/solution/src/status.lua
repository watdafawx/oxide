local M = {}

-- visitors may enter while the park is not full
function M.can_enter(full)
  return not full
end

-- the ride opens only when the staff are ready and the weather is fine
function M.ride_closed(staff_ok, weather_ok)
  return not (staff_ok and weather_ok)
end

return M
