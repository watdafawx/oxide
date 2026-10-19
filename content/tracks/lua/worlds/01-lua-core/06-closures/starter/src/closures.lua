local M = {}

-- each call returns a function: 1, 2, 3, ... independent per counter
function M.make_counter()
  -- TODO: local count outside, return the incrementing function
  return nil
end

-- make_multipliers(3) -> three functions: x*1, x*2, x*3
function M.make_multipliers(n)
  -- TODO: loop 1..n, store function(x) return x * i end
  return nil
end

-- wrap fn(x) so each distinct x is computed only once
function M.memoize(fn)
  -- TODO: a cache table captured by the returned function
  return nil
end

return M
