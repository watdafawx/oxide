local M = {}

-- visitors may enter while the park is not full
function M.can_enter(full)
  return full -- TODO: this is the wrong way round
end

-- the ride opens only when the staff are ready and the weather is fine
function M.ride_closed(staff_ok, weather_ok)
  return nil -- TODO: closed is the opposite of "both are ok"
end

return M
