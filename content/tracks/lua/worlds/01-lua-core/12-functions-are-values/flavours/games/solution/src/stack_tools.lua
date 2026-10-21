local M = {}

-- {30, 90, 40} -> {90, 40, 30}, sorted in place
function M.sort_biggest_first(stacks)
  table.sort(stacks, function(a, b)
    return a > b
  end)
end

-- apply_to_each({1, 2, 3}, double) -> {2, 4, 6}; the original list is not changed
function M.apply_to_each(list, fn)
  local out = {}
  for _, item in ipairs(list) do
    table.insert(out, fn(item))
  end
  return out
end

return M
