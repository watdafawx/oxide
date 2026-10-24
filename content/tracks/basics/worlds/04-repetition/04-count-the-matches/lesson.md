The plan says: rest on every 7th day, run on all the others. How many running days are in a 30-day month? Counting the things that match a rule is a loop with an `if` inside it.

**Count, don't add.** A **counter** is an accumulator that goes up by 1 each time something happens: `count = count + 1`. Start it at 0 before the loop; after the loop it holds how many times it happened.

**Choose inside.** An `if` inside a loop is checked on every round, but its inside runs only on the rounds that match. Loops and decisions combine freely: the inside of a loop can hold anything a function can.

```lua
local hot_days = 0
for day = 1, 7 do
  if temperature_on(day) > 25 then
    hot_days = hot_days + 1
  end
end
```

**A helper.** `is_rest_day(day)` is already in the starter, above your function. It answers true for days 7, 14, 21 and 28. A function may use another function from the same file (`local function` makes one that only this file can see), and you do not need to know how a helper works inside to use it. (It uses `%`, the remainder after dividing: `14 % 7` is 0.)

**Nesting.** Code inside code inside code is called **nesting**, and each level is indented one step further. Three or four levels are fine; deeper than that is a sign to split the work up, which world 5 shows how to do.

## Your turn

- In `running_days`, start a counter at 0.
- Loop over the days from 1 to `days`, and count each day that is not a rest day.
- Return the counter.
