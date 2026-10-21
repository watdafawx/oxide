local M = {}

-- 10 plates, 6 needed -> 4 left. Too few plates raises "not enough plates".
function M.craft(plates, needed)
  if plates < needed then
    error("not enough plates")
  end
  return plates - needed
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_craft(plates, needed)
  return pcall(M.craft, plates, needed)
end

return M
