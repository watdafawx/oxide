local M = {}

-- already written in the last quest: "2 iron-plate" -> 2, "iron-plate"
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

-- "2 iron-plate,1 copper-cable" -> {"2 iron-plate", "1 copper-cable"}
function M.split_commas(line)
  local pieces = {}
  for piece in string.gmatch(line, "[^,]+") do
    table.insert(pieces, piece)
  end
  return pieces
end

-- "2 iron-plate,1 copper-cable" -> {{count = 2, name = "iron-plate"}, {count = 1, name = "copper-cable"}}
function M.parse_recipe(line)
  local recipe = {}
  for _, piece in ipairs(M.split_commas(line)) do
    local count, name = M.parse_ingredient(piece)
    table.insert(recipe, {count = count, name = name})
  end
  return recipe
end

return M
