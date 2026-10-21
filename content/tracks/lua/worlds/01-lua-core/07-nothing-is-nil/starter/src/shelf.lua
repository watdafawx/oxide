local M = {}

-- true only when the slot holds nothing
function M.slot_is_empty(item)
  -- TODO: is item nil?
  return nil
end

-- true for nil and false, false for everything else (0 and "" included)
function M.is_falsy(value)
  -- TODO: flip value with not
  return nil
end

return M
