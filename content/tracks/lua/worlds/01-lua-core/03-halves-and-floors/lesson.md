An omelette takes 2 eggs, and you have 7. How many omelettes is that? Your calculator has to divide, and the answer is not a whole number.

**Divide.** `/` divides: `8 / 2` is 4. Try 7 eggs in your head: `7 / 2`.

**The surprise.** The answer is `3.5`. Lua does not round for you and does not complain. It gives you the exact decimal. You cannot cook half an omelette, so this is a bug waiting to happen.

**One kind of number.** Lua 5.2 has a single kind of number, and decimals are always allowed. Whole results just print without a `.0`, so `8 / 2` shows as `4`.

**Round down.** `math.floor(x)` rounds `x` down to a whole number: `math.floor(3.5)` is 3. This is a **call**: the name, then the value in brackets. The call gives back an answer you can name.

```lua
local exact = eggs / 2                -- 3.5 for 7 eggs
local whole = math.floor(eggs / 2)    -- 3
```

**Down, not up.** Rounding down is the right choice here. The leftover egg stays in the pantry; it does not make a fourth omelette.

## Your turn

- In `omelettes_exact`, return `eggs / 2`.
- In `whole_omelettes`, round that down with `math.floor`.
- Save and check both ticks.
