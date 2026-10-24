local M = {}

-- paint one pixel of the picture the caller handed in
function M.paint(picture, i, colour)
  picture[i] = colour
end

-- a backup for undo: a separate list with the same colours
function M.snapshot(picture)
  local copy = {}
  for i, colour in ipairs(picture) do
    copy[i] = colour
  end
  return copy
end

return M
