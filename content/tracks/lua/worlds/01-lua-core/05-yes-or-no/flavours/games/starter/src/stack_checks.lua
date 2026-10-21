local M = {}

-- a gear needs 2 iron plates
function M.can_craft_gear(plates)
  -- TODO: is plates at least 2?
  return nil
end

-- a full stack of iron plates holds exactly 100
function M.stack_is_full(count)
  -- TODO: is count exactly 100?
  return nil
end

-- anything but 100 is a partial stack
function M.stack_is_partial(count)
  -- TODO: is count different from 100?
  return nil
end

return M
