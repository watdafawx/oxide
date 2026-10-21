local M = {}

-- make_multiplier(3) -> a function; calling it with 5 gives 15
function M.make_multiplier(factor)
  return function(n)
    return n * factor
  end
end

-- each counter counts 1, 2, 3 ... on its own
function M.make_counter()
  local count = 0
  return function()
    count = count + 1
    return count
  end
end

return M
