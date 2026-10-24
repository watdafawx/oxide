local M = {}

-- true when a and b hold the same colours in the same order
function M.same_colours(a, b)
  if #a ~= #b then
    return false
  end
  for i = 1, #a do
    if a[i] ~= b[i] then
      return false
    end
  end
  return true
end

return M
