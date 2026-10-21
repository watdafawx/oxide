local M = {}

-- 5 eggs, 3 needed -> 2 left. Too few eggs raises "not enough eggs".
function M.bake(eggs, needed)
  -- TODO: error("not enough eggs") when eggs < needed, else return eggs - needed
  return nil
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_bake(eggs, needed)
  -- TODO: return pcall(M.bake, eggs, needed)
  return nil
end

return M
