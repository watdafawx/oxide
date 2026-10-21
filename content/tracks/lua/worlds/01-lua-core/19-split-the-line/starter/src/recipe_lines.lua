local M = {}

-- already written in the last quest: "3 eggs" -> 3, "eggs"
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

-- "3 eggs,2 flour" -> {"3 eggs", "2 flour"}
function M.split_commas(line)
  -- TODO: loop with string.gmatch(line, "[^,]+") and table.insert each piece
  return nil
end

-- "3 eggs,2 flour" -> {{count = 3, name = "eggs"}, {count = 2, name = "flour"}}
function M.parse_recipe(line)
  -- TODO: for each piece of split_commas, parse_ingredient it and insert {count = ..., name = ...}
  return nil
end

return M
