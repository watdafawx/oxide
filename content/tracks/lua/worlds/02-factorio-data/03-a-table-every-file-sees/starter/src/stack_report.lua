local data = { raw = { item = {} } }   -- TODO: this local hides the game's global `data`: delete it

local M = {}

-- the stack size of the item prototype called name
function M.stack_of(name)
  -- TODO: read the item called name from data.raw and give back its stack_size
  return nil
end

-- the stack sizes of every item in names, added up
function M.total_stacks(names)
  total = 0   -- TODO: this creates a global; make it a local
  for _, name in ipairs(names) do
    total = total + data.raw.item[name].stack_size
  end
  return total
end

return M
