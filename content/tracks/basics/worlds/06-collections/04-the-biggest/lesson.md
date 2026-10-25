The receipt prints the most expensive item in the basket, and the cheapest. Finding the biggest value in a list is a pattern of its own, with one classic trap.

**Keep a champion.** Walk the list holding the best value so far, the champion. Each round, compare the item with it; if the item beats it, the item becomes the new champion. After the loop, the champion is the biggest of them all.

**Where to start.** The trap is the starting value. Starting at 0 seems fine for prices, but for the cheapest item every real price is above 0, so the answer would be a 0 that is not in the basket. Starting with the first item, `basket[1]`, is always safe: the champion is a real item from the start.

```lua
local oldest = ages[1]
for _, age in ipairs(ages) do
  if age > oldest then
    oldest = age
  end
end
```

**The smallest.** The cheapest item is the same loop with the comparison flipped, `<` instead of `>`. Accumulator, counter, champion: these patterns show up in every language, so learning the shape once pays off everywhere.

**An empty list.** With no items, `basket[1]` is nil and there is no cheapest item. Returning nil then is honest, and the loop simply runs zero times.

## Your turn

- In `most_expensive`, keep a champion that starts at the first price and give back the highest.
- In `cheapest`, do the same for the lowest.
- Save. The tests include a basket where every price is above 0, and an empty one.
