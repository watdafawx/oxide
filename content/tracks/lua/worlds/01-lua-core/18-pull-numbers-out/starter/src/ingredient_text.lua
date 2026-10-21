local M = {}

-- "12 brown-sugar" -> 12, "brown-sugar"; text without a number -> nil, nil
function M.parse_ingredient(text)
  -- TODO: string.match(text, "(%d+) (%S+)"), then return tonumber(count), name
  return 0, "?"
end

return M
