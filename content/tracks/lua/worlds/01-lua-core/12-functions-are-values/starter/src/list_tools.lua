local M = {}

-- {3, 9, 4} -> {9, 4, 3}, sorted in place
function M.sort_biggest_first(counts)
  -- TODO: table.sort with a rule that puts the bigger one first
end

-- apply_to_each({1, 2, 3}, double) -> {2, 4, 6}; the original list is not changed
function M.apply_to_each(list, fn)
  -- TODO: a new list holding fn(item) for every item
  return nil
end

return M
