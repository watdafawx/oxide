**Why now:** the recipe calculator is all numbers, strings and booleans (ingot counts, item names, "is this craftable?"), and Lua 5.2's single number type changes how you halve and compare them.

Lua keeps the rules for **values** short: numbers, strings, booleans, nil.

```lua
local recipe = nil   -- "no value"; reading a missing table field gives nil too
if 0 then print("0 is truthy") end      -- prints! only nil and false are falsy
print(7 / 2)         -- 3.5   (5.2 has one number type: double)
print(10 / 2)        -- 5     (prints without .0 when it is whole)
print("iron" .. "-plate")  -- iron-plate    (.. joins strings; numbers get converted)
print(#"gear")       -- 4     (# is length, for strings and arrays)
```

- `math.floor(x)` rounds down; `x % 2` is the remainder.
- `string.format("%d", 5)` gives `"5"`; `tostring(5)` gives `"5"` too, but `tostring(5.5)` gives `"5.5"`.

## Your turn

- Open `src/numbers.lua`.
- Fill in `is_falsy`, `half`, `floor_half` and `whole`.
