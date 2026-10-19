**Why now:** a recipe is a list of ingredient counts, so the calculator needs to sum them, add to them, sort them and print them as one line.

A Lua table is one structure for list, map and object. Used as a list it is **1-based**, and the `table` library does the work.

```lua
local counts = {10, 20, 30}      -- ingredient counts of one recipe
print(counts[1])        -- 10   (counts[0] is nil!)
print(#counts)          -- 3
counts[#counts + 1] = 40        -- append
table.insert(counts, 50)        -- same thing
local last = table.remove(counts)   -- 50, and counts is shorter
for i, v in ipairs(counts) do print(i, v) end   -- 1 10, 2 20, ...
table.sort(counts, function(a, b) return a > b end)   -- in place, descending
print(table.concat(counts, ", "))   -- "40, 30, 20, 10"
```

`#counts` only counts a run of non-nil values from index 1. A list with a hole in it (`{1, nil, 3}`) has no reliable length. Keep lists dense.

## Your turn

- Open `src/arrays.lua`.
- Fill in `sum`, `append`, `pop`, `sort_desc`, `join`.
