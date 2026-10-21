local M = {}

-- {2, 1, 3} -> 6
function M.total_items(counts)
  -- TODO: a running total, a loop with ipairs, add each count
  return nil
end

-- ({1, 5, 20, 8}, 6) -> 2, because 20 and 8 are above the limit
function M.bulk_items(counts, limit)
  -- TODO: loop; add 1 to a running count when count > limit
  return nil
end

return M
