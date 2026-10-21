local M = {}

-- copy a value and every table inside it
function M.deep_copy(value)
  -- TODO: plain values come back as they are; tables are copied key by key,
  -- calling M.deep_copy on each inner value
  return value
end

return M
