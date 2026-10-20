**Why now:** the recipe calculator reads an inventory of slots, and an empty slot is `nil`. A list with `nil` in the middle quietly breaks `#`, so learn the rules before the numbers go wrong.

`nil` means "no value", and a table cannot store it: `t[2] = nil` *deletes* the key. That leaves a **hole**.

```lua
local inv = {"iron", "copper", "coal"}
inv[2] = nil
print(#inv)                 -- 3 or 1: Lua may pick either. Don't rely on it.

table.remove(inv, 1)        -- shifts the rest down: no hole
print(select("#", nil, nil))   -- 2   real argument count, nils included
print(#{nil, nil})             -- 0   (not reliable)

local p = table.pack("a", nil, nil)
print(p.n)                  -- 3   pack stores the true count in `n`
print(table.unpack(p, 1, p.n))  -- a nil nil
```

- **`...`** in a parameter list (`function M.f(...)`) collects every argument the caller passed. Inside, `select("#", ...)` counts them and `table.pack(...)` stores them, nils and all. `~=` means "not equal": `if x ~= nil then` skips the holes.
- **Rule:** `#` is only trustworthy on a list with no holes.
- To remove from the middle, use `table.remove`. To keep nils on purpose, carry the length yourself (`n`).
- `ipairs` stops at the first `nil`. A numeric `for i = 1, n` does not.

## Your turn

- Open `src/slots.lua`.
- Fill in `remove_slot`, `count_args`, `compact`, `pack_all` and `unpack_all`.
