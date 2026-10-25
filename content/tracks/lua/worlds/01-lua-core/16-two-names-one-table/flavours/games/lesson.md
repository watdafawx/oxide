You want to plan a big build on paper without touching the real chest, so you make a copy. A copy that is not really a copy will quietly wreck the original.

**A second name.** `local copy = chest` does not make a new chest. It gives the one table a second name.

**Watch it go wrong.** Change the table through either name and both names see it, because there is only one table.

```lua
local chest = {coal = 12}
local copy = chest         -- a second name, not a second table
copy.coal = 0
-- chest.coal is now 0 too
```

**Same table, or same contents.** `==` on two tables asks "are these the same table?", not "do they hold the same things?". Two separate tables with equal contents are not `==`.

**A real copy.** Make a new empty `{}` and copy every key and value across with a `pairs` loop. The new table is its own thing. Change it and the original stays as it was.

**If this is new.** Programming 101 explains two names for one table from zero: [Two names, one table](/quest/basics/07-how-the-computer-holds-things/02-two-names-one-table).

## Your turn

- In `copy_chest`, start with a new empty table.
- Loop over the chest with `pairs` and copy each key and value into it.
- Return the new table, not the chest.
