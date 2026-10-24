local M = {}

-- paint one pixel of the picture the caller handed in
function M.paint(picture, i, colour)
  -- TODO
end

-- a backup for undo: a separate list with the same colours
function M.snapshot(picture)
  return picture -- TODO: this hands back the same table, not a copy
end

return M
