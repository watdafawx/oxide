local M = {}

-- {"iron-plate", "stone"} -> "iron-plate"
function M.first_ingredient(recipe)
  return recipe[1]
end

-- {"iron-plate", "stone"} -> 2
function M.ingredient_count(recipe)
  return #recipe
end

-- {"iron-plate", "stone"} -> "stone"
function M.last_ingredient(recipe)
  return recipe[#recipe]
end

return M
