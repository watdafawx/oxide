local M = {}

-- "10 copper-cable" -> 10, "copper-cable"; text without a number -> nil, nil
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

return M
