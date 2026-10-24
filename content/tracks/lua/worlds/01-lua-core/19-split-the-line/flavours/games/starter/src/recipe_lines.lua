local M = {}

-- already written in the last quest: "2 iron-plate" -> 2, "iron-plate"
function M.parse_ingredient(text)
  local count, name = string.match(text, "(%d+) (%S+)")
  return tonumber(count), name
end

-- "2 iron-plate,1 copper-cable" -> {"2 iron-plate", "1 copper-cable"}
function M.split_commas(line)
  -- TODO: loop with string.gmatch over runs of characters that are not a comma,
  -- and insert each piece into a list
  return nil
end

-- "2 iron-plate,1 copper-cable" -> {{count = 2, name = "iron-plate"}, {count = 1, name = "copper-cable"}}
function M.parse_recipe(line)
  -- TODO: for each piece of split_commas, parse_ingredient it and insert {count = ..., name = ...}
  return nil
end

return M
