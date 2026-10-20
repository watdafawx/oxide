local M = {}

-- remove slot i from an inventory list; return what was there
function M.remove_slot(inv, i)
  local old = inv[i]
  inv[i] = nil -- TODO: this leaves a hole. Use table.remove instead
  return old
end

-- number of arguments, nils included
function M.count_args(...)
  -- TODO: #{...} is unreliable with nils. Use select("#", ...)
  return #{...}
end

-- list with holes (length n) -> new dense list, same order
function M.compact(list, n)
  -- TODO: start with out = {}, loop i = 1..n, append list[i] to out when it is not nil
  return nil
end

-- pack arguments into a table that remembers the count in .n
function M.pack_all(...)
  -- TODO: table.pack(...) keeps the nils and the count in .n
  return {...}
end

-- the inverse: all p.n values, nils included
function M.unpack_all(p)
  -- TODO: table.unpack(p, 1, p.n)
  return table.unpack(p)
end

return M
