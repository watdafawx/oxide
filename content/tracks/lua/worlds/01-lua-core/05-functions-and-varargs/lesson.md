**Why now:** the calculator needs helpers that take "however many ingredients there are" and fall back to sensible defaults (one plate per craft, say). Lua functions are loose about argument counts: handy, and also where the bugs live.

```lua
local function bounds(xs)          -- several return values
  return xs[1], xs[#xs]
end
local lo, hi = bounds({1, 5, 9})   -- 1  9

local function craft(item, count)  -- missing arguments are nil
  count = count or 1               -- default with `or`
  return count .. "x " .. item
end

local function total(...)          -- `...` collects the rest
  local sum = 0
  for _, v in ipairs({...}) do sum = sum + v end
  return sum
end
print(total(1, 2, 3))              -- 6
-- select("#", ...) inside a function: how many arguments, nils included
```

- Extra arguments are dropped, missing ones are `nil`. No error either way.
- **Trap:** `x or default` replaces `false` too. `option(false, true)` should stay `false`, so check `x == nil`.
- A call in the middle of an argument list is cut to one value: `f(g(), 1)` uses only g's first return.

## Your turn

- Open `src/fns.lua`.
- Fill in `min_max`, `ingredient_count`, `total` and `option`.
