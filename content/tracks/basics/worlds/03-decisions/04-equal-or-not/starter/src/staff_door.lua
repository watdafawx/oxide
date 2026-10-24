local M = {}

-- the staff door opens for 4721 only
function M.door_opens(code)
  if code = 4721 then -- TODO: Lua will not load this line; read its message
    return true
  end
  return false
end

-- the alarm sounds for every wrong code
function M.sound_alarm(code)
  return false -- TODO: true for any code that is not 4721
end

return M
