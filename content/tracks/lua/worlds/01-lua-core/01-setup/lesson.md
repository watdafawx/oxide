**Why now:** you build a Factorio recipe calculator across this world, and every quest runs in the same Lua that Factorio embeds, so what works here works in a mod.

Lua quests have no compiler and nothing to install: Oxide runs them in a **built-in Lua 5.2**, the same version Factorio embeds.

A quest workspace looks like this:

```text
tests.lua         the spec: one t.test(...) per objective. Read it, do not edit it.
src/numbers.lua   what you write. require("numbers") in tests.lua loads it.
```

Every file under `src/` is a **module**: a table of functions that the file hands back. You will see the same three parts in each starter, and you only fill in the bodies:

```lua
local M = {}                  -- the table that holds this file's functions

function M.double(n)          -- a function called double, taking n
  return n * 2                -- return gives the answer back
end

return M                      -- hand the table to whoever require()s the file
```

Save a file under `src/` and the objectives on the right re-run. A failing check prints the line in tests.lua that expected something else, and what it got:

```text
tests.lua:9: expected 3, got nil
```

Tutorials you find online are often Lua 5.4 or 5.1. The differences that matter here: 5.2 has **no integer type** (every number is a float, so `7 / 2` is `3.5` and `10 / 2` prints as `5`), `goto` exists, and `table.unpack` replaced `unpack`.

## Your turn

Answer the quiz, then start the next quest.
