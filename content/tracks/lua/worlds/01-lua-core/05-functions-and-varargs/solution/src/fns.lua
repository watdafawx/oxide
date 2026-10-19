local M = {}

-- {3, 1, 4} -> 1, 4  (two return values)
function M.min_max(list)
  local lo, hi = list[1], list[1]
  for _, v in ipairs(list) do
    if v < lo then lo = v end
    if v > hi then hi = v end
  end
  return lo, hi
end

-- how many of an ingredient a recipe needs; missing means 1
function M.ingredient_count(n)
  return n or 1
end

-- total(1, 2, 3) -> 6, any number of arguments
function M.total(...)
  local sum = 0
  for _, v in ipairs({...}) do
    sum = sum + v
  end
  return sum
end

-- value unless it is nil, then default (false must survive!)
function M.option(value, default)
  if value == nil then
    return default
  end
  return value
end

return M
