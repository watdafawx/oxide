local M = {}

-- a batch needs 3 eggs
function M.can_make_batch(eggs)
  return eggs >= 3
end

-- a full carton holds exactly 12 eggs
function M.carton_is_full(eggs)
  return eggs == 12
end

-- anything but 12 is a partial carton
function M.carton_is_partial(eggs)
  return eggs ~= 12
end

return M
