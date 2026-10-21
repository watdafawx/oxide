local M = {}

-- "12 brown-sugar" -> 12, "brown-sugar"; text without a number -> nil, nil
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

return M
