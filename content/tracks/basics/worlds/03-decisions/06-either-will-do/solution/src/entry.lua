local M = {}

-- free for toddlers (under 4) and for season-pass holders
function M.free_entry(age, has_pass)
  return age < 4 or has_pass
end

-- the park opens at 9 and closes at 18
function M.is_closed(hour)
  return hour < 9 or hour >= 18
end

return M
