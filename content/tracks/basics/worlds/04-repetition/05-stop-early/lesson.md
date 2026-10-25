The app gives you a badge on the first day of the month that your planned run goes past 10 km. Once it has found that day, looking at the rest of the month is wasted work.

**Stop the loop.** `break` ends a loop at once: Lua jumps to the first line after the loop's `end`, and the rounds that were left never run.

```lua
local found = nil
for day = 1, 31 do
  if rain_on(day) then
    found = day
    break               -- the first rainy day is all we need
  end
end
return found
```

**Return stops too.** Inside a function, a `return` inside a loop ends the loop *and* the function. When the answer is all you need, `return day` right there is the shorter way to stop.

**Nothing found.** If no day matches, the loop runs to the end and `found` is still `nil`. Returning nil, "nothing", is an honest answer, and the code that asked can check for it. The test does exactly that.

**Why it matters.** Stopping early is more than tidy. Searching a million things for the first match takes a millionth of the time when the match is near the front, and programs that search a lot feel much faster for it.

## Your turn

- In `first_day_over`, loop over the days of the month, 1 to 31.
- Stop at the first day where `planned_km(day)` is greater than `km`, and give back that day.
- When no day gets there, give back `nil`.
