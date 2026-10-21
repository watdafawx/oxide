local M = {}

-- {"butter", "sugar"} -> "butter"
function M.first_ingredient(recipe)
  return recipe[1]
end

-- {"butter", "sugar"} -> 2
function M.ingredient_count(recipe)
  return #recipe
end

-- {"butter", "sugar"} -> "sugar"
function M.last_ingredient(recipe)
  return recipe[#recipe]
end

return M
