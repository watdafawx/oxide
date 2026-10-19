**Why now:** Factorio positions are `{x = 3, y = 4}` tables, and the calculator needs to add them and print them. A **metatable** teaches a plain table how to behave: look up methods, react to `+`, print itself, refuse changes.

```lua
local Position = {}
Position.__index = Position            -- missing keys are looked up here

function Position.new(x, y)
  return setmetatable({x = x, y = y}, Position)
end

function Position:manhattan(other)     -- `:` adds `self`
  return math.abs(self.x - other.x) + math.abs(self.y - other.y)
end

Position.__add = function(a, b) return Position.new(a.x + b.x, a.y + b.y) end
Position.__tostring = function(p) return "(" .. p.x .. ", " .. p.y .. ")" end

local a = Position.new(1, 2)
print(a:manhattan(Position.new(4, 6)), tostring(a + a))   -- 7  (2, 4)
```

- `a:f(x)` is sugar for `a.f(a, x)`. Mixing up `.` and `:` is the classic bug.
- `__index` fires only when a key is **missing**. `__newindex` fires when you write a **new** key.
- Other hooks: `__call` (call the table), `__eq`, `__lt`, `__le`, `__len`, `__concat`.
- `rawget` / `rawset` skip the hooks.

## Your turn

- Open `src/position.lua`.
- Fill in `Position.new`, `manhattan`, `__add`, `__tostring` and `read_only`.
