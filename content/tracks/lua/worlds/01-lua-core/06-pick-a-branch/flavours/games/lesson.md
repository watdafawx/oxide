Every item costs a different number of iron plates, so the calculator has to look at the item's name and pick the right number. That is a choice, and Lua writes a choice with `if`.

**One test, one branch.** `if machine == "furnace" then` runs the lines below it only when the test is true. Close it with `end`.

**Another chance.** `elseif` is a second test. Lua tries it only when the first one was false, and you can stack as many as you need.

**The catch-all.** `else` has no test. It runs when every test above it was false, so it is the place for "anything else".

```lua
function M.coal_for(machine)
  if machine == "furnace" then
    return 1
  elseif machine == "boiler" then
    return 2
  else
    return 0
  end
end
```

**First true wins.** Lua walks down the list and stops at the first test that is true. The rest are skipped, even if they would be true too.

**Return ends it.** `return` inside a branch leaves the function right there. That is why each branch can hand back its own answer with no extra variable.

**If this is new.** Programming 101 explains `if` and `else` from zero: [Only sometimes](/quest/basics/03-decisions/01-only-sometimes).

## Your turn

- In `plates_for`, write an `if` for `"iron-gear-wheel"` that returns 2.
- Add an `elseif` for `"pipe"` (1) and one for `"steel-plate"` (5).
- Finish with an `else` that returns 0.
