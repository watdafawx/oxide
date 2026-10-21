local M = {}

-- (3, "eggs") -> "3 eggs"
function M.ingredient_label(count, item)
  -- TODO: count, a space, then item, glued with ..
  return nil
end

-- ("pancakes", 12) -> "pancakes: 12 min"
function M.cooking_line(dish, minutes)
  -- TODO: dish, ": ", minutes, " min"
  return nil
end

return M
