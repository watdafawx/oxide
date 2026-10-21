local M = {}

-- (2, "iron-plate") -> "2x iron-plate"
function M.ingredient_label(count, item)
  return count .. "x " .. item
end

-- ("iron-gear-wheel", 0.5) -> "iron-gear-wheel: 0.5s"
function M.crafting_line(item, seconds)
  return item .. ": " .. seconds .. "s"
end

return M
