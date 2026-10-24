The painter keeps the picture, a list of colours, and a backup for "undo". But the backup keeps changing along with the picture, so undo does nothing. The cause is the difference between copying a number and copying a table.

**Numbers copy.** `local a = 5`, `local b = a`, `b = 6` leaves `a` at 5. Assigning a number copies the value into the new name, and from then on the two are independent. Text works the same way.

**Tables share.** `local b = a` with a table copies only the reference: the note saying where the table lives. Now both names point at the same table, so `b[1] = "red"` changes what `a[1]` shows too. There was only ever one table.

```lua
local today = {"eggs", "milk"}
local backup = today            -- the same list, under a second name
table.insert(backup, "jam")
-- #today is now 3: there was only one list all along
```

**Into functions.** Passing a table to a function passes the reference too. That is why `table.insert(basket, 3)` in the last world could change the caller's basket: the function and the caller were looking at the same list.

**A real copy.** For an independent copy, make a new empty table and copy each item across in a loop. Changing one afterwards leaves the other alone, which is exactly what undo needs.

## Your turn

- In `paint`, store `colour` in slot `i` of `picture`. No `return` needed.
- In `snapshot`, make a new list and copy every colour of `picture` into it.
- Give back the copy. The test changes it and checks that the picture stayed the same.
