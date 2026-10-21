local M = {}

-- how many batches to bake; nobody said, so 1
function M.batches_to_bake(n)
  return n or 1
end

-- the value, or default when the value is nil. false is a real answer!
function M.option(value, default)
  if value == nil then
    return default
  end
  return value
end

return M
