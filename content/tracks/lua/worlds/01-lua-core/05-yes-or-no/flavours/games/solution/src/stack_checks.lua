local M = {}

-- a gear needs 2 iron plates
function M.can_craft_gear(plates)
  return plates >= 2
end

-- a full stack of iron plates holds exactly 100
function M.stack_is_full(count)
  return count == 100
end

-- anything but 100 is a partial stack
function M.stack_is_partial(count)
  return count ~= 100
end

return M
