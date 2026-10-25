Before every sprint the app counts down: 5 4 3 2 1 go. And on long runs it reminds you to drink every 15 minutes. Loops can count backwards, and in bigger steps than one.

**A step.** A third number sets the **step**: `for i = 0, 20, 5 do` gives 0, 5, 10, 15 and 20. Without a third number, the step is 1.

**Backwards.** A negative step counts down: `for i = 5, 1, -1 do` gives 5, 4, 3, 2, 1. Leave out the `-1` and Lua tries to count up from 5 to 1, sees that 5 is already past 1, and runs zero times.

```lua
local marks = ""
for m = 0, 300, 100 do
  marks = marks .. m .. " "
end
return marks                  -- "0 100 200 300 "
```

**Build text the same way.** An accumulator does not have to be a number. Start with empty text, `""`, and glue a piece onto it every round, just as a total adds a number every round.

**Where it ends.** The loop stops at the end number or before it, never after: `for i = 1, 10, 4` gives 1, 5 and 9, because 13 would be past 10. When in doubt, trace the first two rounds and the last one.

## Your turn

- In `countdown`, glue each number from `from` down to 1 onto the text, each followed by a space.
- Then add `go` at the end.
- In `water_breaks`, count one break at 15, 30, 45 and so on, up to `minutes`.
