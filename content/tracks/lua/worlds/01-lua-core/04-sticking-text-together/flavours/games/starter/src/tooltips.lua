local M = {}

-- (2, "iron-plate") -> "2x iron-plate"
function M.ingredient_label(count, item)
  -- TODO: count, "x ", then item, glued with ..
  return nil
end

-- ("iron-gear-wheel", 0.5) -> "iron-gear-wheel: 0.5s"
function M.crafting_line(item, seconds)
  -- TODO: item, ": ", seconds, "s"
  return nil
end

return M
