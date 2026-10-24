The till's total is every price in the basket, added up. A basket can hold 2 items or 200, so the code cannot name each slot. One loop covers a list of any size.

**Count through.** You already know `for i = 1, n do`. With `#basket` as the end, the loop visits every slot, and `basket[i]` is the item for this round. That works, but Lua has a shorter way.

**ipairs.** Visiting every item in order is so common that Lua has a shortcut: `for i, price in ipairs(basket) do` hands you the slot number and the item together, each round. `ipairs` means "index pairs".

```lua
local names = ""
for i, name in ipairs(queue) do
  names = names .. i .. ". " .. name .. " "
end
-- "1. Ana 2. Ben 3. Cleo "
```

**Skip a name.** When you only need the item, call the slot number `_`: `for _, price in ipairs(basket) do`. `_` is an ordinary name, but programmers use it to say "I will not use this one".

**The pattern.** Accumulator before the loop, one item per round, the answer after the loop. You built it with numbers in world 4; with lists it is the most common loop in all of programming. Rust writes it `for price in basket`, Java `for (double price : basket)`.

## Your turn

- In `basket_total`, add up every price in `basket`.
- In `count_over`, count the prices that are greater than `limit`.
- Save. The tests include an empty basket.
