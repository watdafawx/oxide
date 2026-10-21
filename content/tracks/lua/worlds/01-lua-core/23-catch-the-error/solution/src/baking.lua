local M = {}

-- 5 eggs, 3 needed -> 2 left. Too few eggs raises "not enough eggs".
function M.bake(eggs, needed)
  if eggs < needed then
    error("not enough eggs")
  end
  return eggs - needed
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_bake(eggs, needed)
  return pcall(M.bake, eggs, needed)
end

return M
