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

- Compare with `==` (equal) and `~=` (not equal); combine tests with `or`, `and`, `not`. `v == nil or v == false` is true when either holds.
- `math.floor(x)` rounds down; `x % 2` is the remainder.
- `if a then ... elseif b then ... else ... end` picks one branch; `x ^ y` is "x to the power y" (`2 ^ 3` is `8`).
- `string.format("%d", 5)` gives `"5"` (`%s` slots in any string, so `string.format("%s has %d", "iron", 3)` is `"iron has 3"`; `("%d"):format(5)` is the same call); `tostring(5)` gives `"5"` too, but `tostring(5.5)` gives `"5.5"`.

## Your turn

- Open `src/numbers.lua`.
- Fill in `is_falsy`, `half`, `floor_half` and `whole`.
