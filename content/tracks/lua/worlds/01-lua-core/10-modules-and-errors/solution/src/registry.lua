local recipes = {}

local M = {}

-- store a recipe under its name
function M.register(name, recipe)
  recipes[name] = recipe
end

-- the recipe registered under name, or nil
function M.get(name)
  return recipes[name]
end

return M
