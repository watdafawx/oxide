local M = {}

-- a recipe with its own methods
function M.new(name, counts)
  local recipe = {name = name, counts = counts}

  -- 3 + 1 + 2 -> 6
  function recipe:total_items()
    local total = 0
    for _, count in ipairs(self.counts) do
      total = total + count
    end
    return total
  end

  -- "pancakes: 6 items"
  function recipe:describe()
    return self.name .. ": " .. self:total_items() .. " items"
  end

  return recipe
end

return M
