local M = {}

-- {3, 1, 4} -> 1, 4  (two return values)
function M.min_max(list)
  -- TODO: walk the list, return lo, hi
  return nil
end

-- how many of an ingredient a recipe needs; missing means 1
function M.ingredient_count(n)
  -- TODO: default with `or`
  return nil
end

-- total(1, 2, 3) -> 6, any number of arguments
function M.total(...)
  -- TODO: loop over {...}
  return nil
end

-- value unless it is nil, then default (false must survive!)
function M.option(value, default)
  -- TODO: `value or default` is wrong for false. Compare to nil
  return nil
end

return M
