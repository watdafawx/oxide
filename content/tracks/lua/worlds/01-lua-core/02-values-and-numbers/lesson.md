Every Factorio prototype is a table of **values**: numbers, strings, booleans, nil. Lua keeps the rules short.

```lua
local x = nil        -- "no value"; reading a missing table field gives nil too
if 0 then print("0 is truthy") end      -- prints! only nil and false are falsy
print(7 / 2)         -- 3.5   (5.2 has one number type: double)
print(10 / 2)        -- 5     (prints without .0 when it is whole)
print("a" .. "b")    -- ab    (.. joins strings; numbers get converted)
print(#"hello")      -- 5     (# is length, for strings and arrays)
```

- `math.floor(x)` rounds down; `x % 2` is the remainder.
- `string.format("%d", 5)` gives `"5"`; `tostring(5)` gives `"5"` too, but `tostring(5.5)` gives `"5.5"`.

## Your turn

- Open `src/numbers.lua`.
- Fill in `is_falsy`, `half`, `floor_half` and `whole`.
