local M = {}

-- a batch needs 3 eggs
function M.can_make_batch(eggs)
  -- TODO: is eggs at least 3?
  return nil
end

-- a full carton holds exactly 12 eggs
function M.carton_is_full(eggs)
  -- TODO: is eggs exactly 12?
  return nil
end

-- anything but 12 is a partial carton
function M.carton_is_partial(eggs)
  -- TODO: is eggs different from 12?
  return nil
end

return M
