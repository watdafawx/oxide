A gear takes 2 iron plates, and the chest holds 7. How many gears is that? Your calculator has to divide, and the answer is not a whole number.

**Divide.** `/` divides: `8 / 2` is 4. Try 7 plates in your head: `7 / 2`.

**The surprise.** The answer is `3.5`. Lua does not round for you and does not complain. It gives you the exact decimal. An assembler cannot craft half a gear, so this is a bug waiting to happen.

**One kind of number.** Lua 5.2 has a single kind of number, and decimals are always allowed. Factorio's Lua works the same way. Whole results just print without a `.0`, so `8 / 2` shows as `4`.

**Round down.** `math.floor(x)` rounds `x` down to a whole number: `math.floor(3.5)` is 3. This is a **call**: the name, then the value in brackets. The call gives back an answer you can name.

```lua
local exact = plates / 2                -- 3.5 for 7 plates
local whole = math.floor(plates / 2)    -- 3
```

**Down, not up.** Rounding down is the right choice here. The leftover plate stays in the chest; it does not make a fourth gear.

## Your turn

- In `gears_exact`, return `plates / 2`.
- In `whole_gears`, round that down with `math.floor`.
- Save and check both ticks.
