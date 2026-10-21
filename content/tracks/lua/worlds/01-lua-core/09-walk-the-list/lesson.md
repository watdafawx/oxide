The shopping list holds how much of each thing to buy: `{2, 1, 3}`. The calculator wants the total, and that means looking at every item in turn.

**Do it for each.** `for _, count in ipairs(counts) do ... end` runs the lines inside once for every item, in order. Lines inside the loop are the ones that repeat.

**Two names.** `ipairs` hands out two things each round: the position and the value. The loop gives them names. `_` is the usual name for "I do not need this one".

**Keep a running value.** To collect an answer, make a name before the loop and change it inside. The name survives from one round to the next.

```lua
local biggest = 0
for _, count in ipairs(counts) do
  if count > biggest then
    biggest = count
  end
end
```

**Choose inside.** An `if` inside the loop acts on only some of the items. Here it updates `biggest` only when this item beats it.

## Your turn

- In `total_items`, start `local total = 0`, loop with `ipairs`, and add each `count`.
- In `bulk_items`, count how many `counts` are bigger than `limit`.
- Save. Both objectives turn green.
