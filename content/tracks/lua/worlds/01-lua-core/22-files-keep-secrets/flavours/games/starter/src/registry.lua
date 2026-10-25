local M = {}

-- TODO: this table is public because it hangs on M.
-- Make it a local table named recipes, declared above M instead, and delete this line.
M.recipes = {}

-- store a recipe under its name
function M.register(name, recipe)
  -- TODO: store the recipe under its name
end

-- the recipe stored under name, or nil
function M.get(name)
  -- TODO: the recipe stored under name
  return nil
end

return M
