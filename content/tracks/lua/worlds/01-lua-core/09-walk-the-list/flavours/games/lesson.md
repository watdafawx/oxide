A recipe's ingredient counts are a list: `{2, 1, 3}`. The calculator wants the total number of items, and that means looking at every count in turn.

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

**If this is new.** Programming 101 explains loops from zero: [Count with for](/quest/basics/04-repetition/02-count-with-for).

## Your turn

- In `total_ingredients`, start `local total = 0`, loop with `ipairs`, and add each `count`.
- In `bulk_ingredients`, count how many `counts` are bigger than `limit`.
- Save. Both objectives turn green.
