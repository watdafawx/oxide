local M = {}

-- {1, 2, 3} -> 6
function M.sum(xs)
  local total = 0
  for _, v in ipairs(xs) do
    total = total + v
  end
  return total
end

-- add v at the end of xs
function M.append(xs, v)
  xs[#xs + 1] = v
end

-- remove and return the last element
function M.pop(xs)
  return table.remove(xs)
end

-- sort xs in place, biggest first
function M.sort_desc(xs)
  table.sort(xs, function(a, b) return a > b end)
end

-- {"a", "b"} -> "a, b"
function M.join(xs)
  return table.concat(xs, ", ")
end

return M
