local M = {}

-- true only when the slot holds nothing
function M.slot_is_empty(stack)
  return stack == nil
end

-- true for nil and false, false for everything else (0 and "" included)
function M.is_falsy(value)
  return not value
end

return M
