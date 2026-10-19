local recipes = {}

local M = {}

M.recipes = recipes -- TODO: this makes the table public. Delete the line

-- store a recipe under its name
function M.register(name, recipe)
  -- TODO
end

-- the recipe registered under name, or nil
function M.get(name)
  -- TODO
  return nil
end

return M
