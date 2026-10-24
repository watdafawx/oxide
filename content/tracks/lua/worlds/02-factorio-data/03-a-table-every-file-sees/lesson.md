Before adding anything, the Kit Mod should look at what is already there: how big is a stack of iron plates? The answer sits in `data.raw`, and the table `data` is the first name in this track you use without ever writing `local`.

**No local, shared.** In the last world every name started with `local`, which keeps it inside its file or function. A name given a value without `local` the first time is a **global**: one name shared by every file in the program.

**Why you avoided them.** Two files that both use a global `total` overwrite each other's number. And a typo like `totl = 1` quietly makes a new global instead of an error. That is why the habit was "always write `local`".

**The one you are handed.** Factorio creates a few globals before your file runs, and `data` is the big one: the table of every prototype. Every mod reads and writes this same table, which is exactly why it is global. You use it without creating it.

**Don't hide it.** `local data = {}` makes a new, empty local table that hides the global one for the rest of the file. Your code then reads an empty table, and the game never sees anything you add to it.

```lua
-- no `local data` anywhere: `data` is the game's global table
local M = {}

function M.crafting_time(name)
  return data.raw.recipe[name].energy_required
end

return M
```

## Your turn

- Delete the `local data` line at the top of `stack_report.lua`.
- Make `stack_of` give back the stack size of the item called `name`.
- In `total_stacks`, make `total` a local so it does not leak out as a global.
