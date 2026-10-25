Each time the till scans an item, its price goes onto the end of the basket. And when the scanner misreads a price, the cashier corrects that one slot. A list's size is not fixed: it changes while the program runs.

**Change a slot.** `basket[2] = 1.5` stores a new value in slot 2, the same way `price = 1.5` stores one into a name. The other slots stay as they were.

**Add to the end.** `table.insert(basket, 3)` puts 3 after the last item, so `#basket` grows by one. `table` is Lua's built-in toolbox for lists, and `insert` is one of its tools.

```lua
local queue = {"Ana", "Ben"}
table.insert(queue, "Cleo")     -- Ana, Ben, Cleo
queue[1] = "Dev"                -- Dev, Ben, Cleo
```

**Start empty.** `{}` is a list with nothing in it yet, and `#{}` is 0. Most lists start out empty and fill up as things happen, like a basket at the start of a sale.

**The list itself changes.** `table.insert(basket, 3)` does not make a new list; it changes the one you gave it. That is why your functions here return nothing, and the test still sees the change in its own basket. World 7 explains why that works.

## Your turn

- In `scan`, put `price` at the end of `basket`.
- In `fix_price`, store `price` in slot `slot` of `basket`.
- Neither function needs a `return`: the test looks at the basket it handed in.
