A Lua table is one data structure for everything: list, map, object. Used as a list, it is **1-based** and the `table` library does the work.

```lua
local xs = {10, 20, 30}
print(xs[1])            -- 10   (xs[0] is nil!)
print(#xs)              -- 3
xs[#xs + 1] = 40        -- append
table.insert(xs, 50)    -- same thing
local last = table.remove(xs)   -- 50, and xs is shorter
for i, v in ipairs(xs) do print(i, v) end   -- 1 10, 2 20, ...
table.sort(xs, function(a, b) return a > b end)   -- in place, descending
print(table.concat(xs, ", "))   -- "40, 30, 20, 10"
```

`#xs` only counts a run of non-nil values from index 1. A list with a hole in it (`{1, nil, 3}`) has no reliable length. Keep lists dense.

## Your turn

- Open `src/arrays.lua`.
- Fill in `sum`, `append`, `pop`, `sort_desc`, `join`.
