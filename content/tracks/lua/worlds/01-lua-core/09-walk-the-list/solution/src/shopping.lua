local M = {}

-- {2, 1, 3} -> 6
function M.total_items(counts)
  local total = 0
  for _, count in ipairs(counts) do
    total = total + count
  end
  return total
end

-- ({1, 5, 20, 8}, 6) -> 2, because 20 and 8 are above the limit
function M.bulk_items(counts, limit)
  local n = 0
  for _, count in ipairs(counts) do
    if count > limit then
      n = n + 1
    end
  end
  return n
end

return M
