local M = {}

-- (3, "eggs") -> "3 eggs"
function M.ingredient_label(count, item)
  return count .. " " .. item
end

-- ("pancakes", 12) -> "pancakes: 12 min"
function M.cooking_line(dish, minutes)
  return dish .. ": " .. minutes .. " min"
end

return M
