local M = {}

-- countdown(3) -> "3 2 1 go"
function M.countdown(from)
  local text = ""
  for i = from, 1, -1 do
    text = text .. i .. " "
  end
  return text .. "go"
end

-- a water break at 15, 30, 45 ... minutes: how many during a run of `minutes`?
function M.water_breaks(minutes)
  local count = 0
  for m = 15, minutes, 15 do
    count = count + 1
  end
  return count
end

return M
