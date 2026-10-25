In this world you build a recipe calculator for a Factorio factory: give it a recipe, get back what it costs and whether the chest has enough. Before the first feature, learn how a Lua quest runs.

**Two files.** Every quest gives you two files. `tests.lua` is the spec: it says what each function must do. The file under `src/` is yours. You edit that one and never touch `tests.lua`.

**Save and watch.** Every time you save, Oxide runs the tests. Each objective above the code turns green or stays red. A red one shows the line of `tests.lua` that failed, like `tests.lua:9: expected 3, got nil`.

**A module.** The file you edit is a **module**: a table of functions that the file hands back. A Factorio mod is a pile of these. The starter already holds the outline, and you only fill in the bodies:

```lua
local M = {}                  -- the table that holds this file's functions

function M.double(n)          -- a function called double, taking n
  return n * 2                -- return gives the answer back
end

return M                      -- hand the table to whoever asks for the file
```

**Empty on purpose.** In a starter the body says `-- TODO` and `return nil`. `nil` is Lua's word for "nothing", and a function that gives back nothing is why every objective starts red. Your job is to replace the `nil`.

**Lua 5.2.** Factorio embeds Lua 5.2, and so does Oxide, so what works here works in your mod. Tutorials online are often 5.1 or 5.4. If one disagrees with a quest, trust the quest.

**If you are new to coding.** [Programming 101](/quest/basics/01-what-a-program-is/01-instructions-in-order) teaches the ideas every language shares, once and in plain words: variables, choices, loops, functions, lists. It is optional and uses Lua; skip it if you have programmed before.

**If this is new.** Programming 101 explains tests and functions from zero: [What a test is](/quest/basics/01-what-a-program-is/06-what-a-test-is) and [Define and call](/quest/basics/05-functions/01-define-and-call).

## Your turn

Answer the quiz, then start the next quest.
