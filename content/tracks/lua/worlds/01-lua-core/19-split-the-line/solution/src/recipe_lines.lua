local M = {}

-- already written in the last quest: "3 eggs" -> 3, "eggs"
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

-- "3 eggs,2 flour" -> {"3 eggs", "2 flour"}
function M.split_commas(line)
  local pieces = {}
  for piece in string.gmatch(line, "[^,]+") do
    table.insert(pieces, piece)
  end
  return pieces
end

-- "3 eggs,2 flour" -> {{count = 3, name = "eggs"}, {count = 2, name = "flour"}}
function M.parse_recipe(line)
  local recipe = {}
  for _, piece in ipairs(M.split_commas(line)) do
    local count, name = M.parse_ingredient(piece)
    table.insert(recipe, {count = count, name = name})
  end
  return recipe
end

return M
