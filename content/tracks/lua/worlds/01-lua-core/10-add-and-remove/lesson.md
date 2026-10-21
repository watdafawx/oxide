The shopping list grows when you remember something and shrinks when you cross it off. The calculator needs both moves, and neither should leave a mess behind.

**Add to the end.** `table.insert(list, "sugar")` puts `"sugar"` after the last item. `#list` is now one bigger.

**Take one out.** `table.remove(list, 2)` removes the item at index 2 and **hands it back**, so you can keep it in a name.

```lua
local list = {"flour", "eggs", "milk"}
table.insert(list, "sugar")         -- flour, eggs, milk, sugar
local gone = table.remove(list, 2)  -- "eggs"; the list is flour, milk, sugar
```

**The gap closes.** After a `remove`, every item behind the removed one moves down one place. The list stays packed with no gaps.

**Why not `= nil`?** `list[2] = nil` also empties slot 2, but nothing moves down. That leaves a **hole**, and with a hole `#list` can report the wrong count. Use `table.remove`.

## Your turn

- In `add_item`, add the item at the end of the list.
- In `cross_off`, remove slot `i` and return what was there.
- Save. Both objectives turn green.
