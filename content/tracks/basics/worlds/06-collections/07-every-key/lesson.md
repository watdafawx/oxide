At closing time the till needs two numbers: how many items are on the shelves, and what they are worth, count times price for every product. A map has no slot numbers to count through, so it needs its own kind of loop.

**Every key.** `for item, count in pairs(stock) do` visits every key of a map and hands you the key and its value, each round. `pairs` is to maps what `ipairs` is to lists.

**No order.** `pairs` gives the keys in no promised order, and the order can change between runs and between Lua versions. Totals and counts do not care about order. When you do need a fixed order, keep a list of the keys as well.

```lua
local seats_total = 0
for zone, free in pairs(seats) do
  seats_total = seats_total + free
end
```

**Two tables, one key.** The key from one map can look up the same thing in another. Inside the loop, `prices[item]` finds the price of the product `stock` is talking about. Keys connect tables the way names connect people to phone numbers.

**ipairs or pairs.** Lists get `ipairs`: slots 1, 2, 3 in order. Maps get `pairs`: every key, in any order. `ipairs` on a map visits nothing at all, because a map has no slot 1.

## Your turn

- In `items_in_stock`, add up every count in `stock`.
- In `stock_value`, add up count times price for every product, looking the price up in `prices`.
- Save. Every product in the tests has a price.
