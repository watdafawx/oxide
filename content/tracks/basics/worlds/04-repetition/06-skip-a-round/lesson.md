Your running log marks the days you were ill with a distance of -1. The weekly average should leave those days out completely. They are not 0 km days; they should not be counted at all, and the loop still has to go on to the next day.

**Skip, don't stop.** Sometimes one round should be skipped while the loop carries on. Rust, Java and most languages have a word for that, `continue`: "skip the rest of this round and start the next one". `break` would end the whole loop instead.

**Lua's way.** Lua 5.2 has no `continue`. Instead, wrap the work in an `if` that only lets the good rounds through. On a bad round the `if` does nothing, and the loop simply moves on to the next one.

```lua
local open_hours = 0
for hour = 9, 17 do
  if hour ~= 13 then          -- 13:00 is lunch: skip that hour
    open_hours = open_hours + 1
  end
end
```

**Two at once.** One loop can keep several accumulators side by side: a total and a count. The average is the total divided by the count, worked out once, after the loop.

**Check before dividing.** If every day was skipped, the count is 0, and dividing by 0 gives a nonsense number (Lua prints it as `nan`, and some languages crash instead). Check for a count of 0 first and return 0.

## Your turn

- In `average_km`, keep a total and a count of the days that are not -1.
- Use the helper `logged_km(day)` for each day from 1 to `days`.
- Return the total divided by the count, or 0 when nothing was counted.
