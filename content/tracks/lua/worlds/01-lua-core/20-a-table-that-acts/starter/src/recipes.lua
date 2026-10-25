local M = {}

-- a recipe with its own methods
function M.new(name, counts)
  local recipe = {name = name, counts = counts}

  -- 3 + 1 + 2 -> 6
  function recipe:total_items()
    -- TODO: add up self.counts with a loop
    return nil
  end

  -- "pancakes: 6 items"
  function recipe:describe()
    -- TODO: the name, a colon and space, the total from the other method, then " items"
    return nil
  end

  return recipe
end

return M
