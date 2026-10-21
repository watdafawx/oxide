local M = {}

-- 10 plates, 6 needed -> 4 left. Too few plates raises "not enough plates".
function M.craft(plates, needed)
  -- TODO: error("not enough plates") when plates < needed, else return plates - needed
  return nil
end

-- same job, but never stops: gives true and what is left, or false and the message
function M.try_craft(plates, needed)
  -- TODO: return pcall(M.craft, plates, needed)
  return nil
end

return M
