local M = {}

-- each call returns a function: 1, 2, 3, ... independent per counter
function M.make_counter()
  local count = 0
  return function()
    count = count + 1
    return count
  end
end

-- make_multipliers(3) -> three functions: x*1, x*2, x*3
function M.make_multipliers(n)
  local fns = {}
  for i = 1, n do
    fns[i] = function(x) return x * i end
  end
  return fns
end

-- wrap fn(x) so each distinct x is computed only once
function M.memoize(fn)
  local cache = {}
  return function(x)
    if cache[x] == nil then
      cache[x] = fn(x)
    end
    return cache[x]
  end
end

return M
