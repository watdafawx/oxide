local M = {}

-- copy a value and every table inside it
function M.deep_copy(value)
  if type(value) ~= "table" then
    return value
  end
  local out = {}
  for key, inner in pairs(value) do
    out[key] = M.deep_copy(inner)
  end
  return out
end

return M
