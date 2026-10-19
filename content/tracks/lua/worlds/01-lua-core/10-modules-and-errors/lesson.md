**Why now:** the calculator is now several files. Each hides its helpers, reports failures as data, and must not leak globals. Every Factorio mod is exactly this: a pile of modules, with `pcall` between them.

```lua
-- registry.lua
local recipes = {}                 -- private: nobody else can touch it
local M = {}
function M.register(name, r) recipes[name] = r end
function M.get(name) return recipes[name] end
return M                           -- `require("registry")` returns M

-- elsewhere
local ok, err = pcall(function()
  error({code = "E_NO_PLATES", need = 3})   -- any value, tables too
end)
print(ok, err.code)                -- false  E_NO_PLATES

error("oops")                      -- adds "file:line:" in front
error("oops", 0)                   -- exactly "oops"
assert(x, "x is missing")          -- error if x is nil/false, else returns x
```

- `require` loads a file once and caches what it returns. **Public API = that returned table.** Locals stay private.
- **Globals leak** when you forget `local`. In Lua 5.2 every chunk runs in `_ENV`, and you can swap it:

```lua
local env = setmetatable({}, {__newindex = function(_, k) print("leak:", k) end})
load("oops = 1", "chunk", "t", env)()    -- prints  leak: oops
```

- `xpcall(fn, handler)` is `pcall` plus a handler that sees the error first.

## Your turn

- Open `src/registry.lua` and `src/guard.lua`.
- Hide the registry's `recipes` table, then fill in `error_code`, `with_context` and `find_globals`.
