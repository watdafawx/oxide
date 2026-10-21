local M = {}

-- a recipe with its own methods
function M.new(name, counts)
  local recipe = {name = name, counts = counts}

  -- 1 + 3 -> 4
  function recipe:total_items()
    local total = 0
    for _, count in ipairs(self.counts) do
      total = total + count
    end
    return total
  end

  -- "electronic-circuit: 4 items"
  function recipe:describe()
    return self.name .. ": " .. self:total_items() .. " items"
  end

  return recipe
end

return M
