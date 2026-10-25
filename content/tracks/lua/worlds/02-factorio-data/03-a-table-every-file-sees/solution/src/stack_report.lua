local M = {}

-- the stack size of the item prototype called name
function M.stack_of(name)
  return data.raw.item[name].stack_size
end

-- the stack sizes of every item in names, added up
function M.total_stacks(names)
  local total = 0
  for _, name in ipairs(names) do
    total = total + data.raw.item[name].stack_size
  end
  return total
end

return M
