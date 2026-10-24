local M = {}

-- 5 eggs, 3 needed -> 2 left. Too few eggs raises "not enough eggs".
function M.bake(eggs, needed)
  -- TODO: raise the error "not enough eggs" when there are too few, else return the eggs left over
  return nil
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_bake(eggs, needed)
  -- TODO: call M.bake in protected mode, so an error comes back as false and the message
  return nil
end

return M
