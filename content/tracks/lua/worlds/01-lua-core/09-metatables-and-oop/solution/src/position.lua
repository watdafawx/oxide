local M = {}

local Position = {}
Position.__index = Position

-- a new Position {x, y} that uses Position's methods
function M.new(x, y)
  return setmetatable({x = x, y = y}, Position)
end

-- |dx| + |dy|
function Position:manhattan(other)
  return math.abs(self.x - other.x) + math.abs(self.y - other.y)
end

-- position + position -> new Position
Position.__add = function(a, b)
  return M.new(a.x + b.x, a.y + b.y)
end

-- tostring(position) -> "(1, 2)"
Position.__tostring = function(p)
  return ("(%s, %s)"):format(p.x, p.y)
end

-- a proxy that reads from t but errors "read-only" on every write
function M.read_only(t)
  return setmetatable({}, {
    __index = t,
    __newindex = function()
      error("read-only", 2)
    end,
  })
end

return M
