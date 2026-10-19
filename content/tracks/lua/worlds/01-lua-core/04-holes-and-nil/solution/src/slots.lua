local M = {}

-- remove slot i from an inventory list; return what was there
function M.remove_slot(inv, i)
  return table.remove(inv, i)
end

-- number of arguments, nils included
function M.count_args(...)
  return select("#", ...)
end

-- list with holes (length n) -> new dense list, same order
function M.compact(list, n)
  local out = {}
  for i = 1, n do
    if list[i] ~= nil then
      out[#out + 1] = list[i]
    end
  end
  return out
end

-- pack arguments into a table that remembers the count in .n
function M.pack_all(...)
  return table.pack(...)
end

-- the inverse: all p.n values, nils included
function M.unpack_all(p)
  return table.unpack(p, 1, p.n)
end

return M
