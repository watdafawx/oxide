local M = {}

-- 10 plates, 6 needed -> 4 left. Too few plates raises "not enough plates".
function M.craft(plates, needed)
  -- TODO: raise the error "not enough plates" when there are too few, else return the plates left over
  return nil
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_craft(plates, needed)
  -- TODO: call M.craft in protected mode, so an error comes back as false and the message
  return nil
end

return M
