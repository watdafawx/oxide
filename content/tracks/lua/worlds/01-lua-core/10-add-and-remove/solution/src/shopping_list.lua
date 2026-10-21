local M = {}

-- put item at the end of list
function M.add_item(list, item)
  table.insert(list, item)
end

-- remove slot i, close the gap, and return what was there
function M.cross_off(list, i)
  return table.remove(list, i)
end

return M
