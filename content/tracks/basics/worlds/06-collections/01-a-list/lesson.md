A customer's basket holds several prices: 2.5, 1 and 4. Giving each one its own name, `price1`, `price2`, `price3`, breaks the moment a basket holds one more item. The till needs one name for the whole row.

**One name, many values.** `local basket = {2.5, 1, 4}` stores three values, in order, under one name. A value that holds others in order like this is a **list** (also called an array). Curly brackets make it, and commas separate the items.

**Numbered slots.** Every item sits in a numbered slot: `basket[1]` is the first item, 2.5, and `basket[3]` is the third. The number in the square brackets is called the **index**.

```lua
local queue = {"Ana", "Ben", "Cleo"}
local first = queue[1]          -- "Ana"
local size = #queue             -- 3
local last = queue[#queue]      -- "Cleo"
```

**Counting from 1.** Lua counts slots from 1, the way people do. Rust, Java and most other languages count from 0, so there the first item is `basket[0]`. When you switch languages, this is the first thing to check again.

**How many.** `#basket` gives the number of items, here 3. Because the last slot's number equals the count, `basket[#basket]` is always the last item, however long the list gets.

**Past the end.** `basket[10]` on a 3-item list is not an error in Lua: it is `nil`, nothing. Rust stops the program instead, and Java throws an error. In every language, an index past the end means a bug somewhere.

## Your turn

- In `first_item`, give back the first price in the basket.
- In `item_count`, give back how many prices there are.
- In `second_to_last`, give back the price just before the last one.
