local M = {}

-- "12 brown-sugar" -> 12, "brown-sugar"; text without a number -> nil, nil
function M.parse_ingredient(text)
  -- TODO: string.match with a pattern that captures the digits and the word after the space,
  -- then return the count turned into a number, and the name
  return 0, "?"
end

return M
