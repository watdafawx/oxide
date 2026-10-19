Lua quests have no compiler and nothing to install: Oxide runs them in a **built-in Lua 5.2**, the same version Factorio embeds.

A quest workspace looks like this:

```text
tests.lua         the spec: one t.test(...) per objective. Read it, do not edit it.
src/numbers.lua   what you write. require("numbers") in tests.lua loads it.
```

Save a file under `src/` and the objectives on the right re-run. A failing check prints the line in tests.lua that expected something else, and what it got:

```text
tests.lua:9: expected 3, got nil
```

Tutorials you find online are often Lua 5.4 or 5.1. The differences that matter here: 5.2 has **no integer type** (every number is a float, so `7 / 2` is `3.5` and `10 / 2` prints as `5`), `goto` exists, and `table.unpack` replaced `unpack`.

## Your turn

Answer the quiz, then start the next quest.
