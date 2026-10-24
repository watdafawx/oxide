local M = {}

-- the big coaster: at least 140 cm tall AND at least 10 years old
function M.may_ride(height, age)
  return height >= 140 -- TODO: this checks one rule; the sign has two
end

-- send visitors to the fast lane when they have a pass and the wait is over 30 minutes
function M.use_fast_lane(has_pass, wait)
  return nil -- TODO
end

return M
