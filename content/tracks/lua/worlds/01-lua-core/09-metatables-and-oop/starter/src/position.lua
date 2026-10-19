local M = {}

local Position = {}
-- TODO: Position.__index = Position

-- a new Position {x, y} that uses Position's methods
function M.new(x, y)
  -- TODO: setmetatable
  return {x = x, y = y}
end

-- |dx| + |dy|
function Position:manhattan(other)
  -- TODO
  return nil
end

-- position + position -> new Position
-- TODO: Position.__add = function(a, b) ... end

-- tostring(position) -> "(1, 2)"
-- TODO: Position.__tostring = function(p) ... end

-- a proxy that reads from t but errors "read-only" on every write
function M.read_only(t)
  -- TODO: empty table + metatable with __index and __newindex
  return t
end

return M
