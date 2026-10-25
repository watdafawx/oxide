local M = {}

-- the staff door opens for 4721 only
function M.door_opens(code)
  if code == 4721 then
    return true
  end
  return false
end

-- the alarm sounds for every wrong code
function M.sound_alarm(code)
  return code ~= 4721
end

return M
