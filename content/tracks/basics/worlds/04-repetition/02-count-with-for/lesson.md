This month's plan: 1 push-up on day 1, 2 on day 2, and so on. How many push-ups is that after a number of days? This time the number of rounds is known before the loop starts.

**Count for me.** `for day = 1, 30 do ... end` runs the inside 30 times. The first time `day` is 1, then 2, and so on up to 30. Lua makes the name `day`, sets it each round, and throws it away after the `end`.

**Running total.** To add things up, make a name before the loop, starting at 0, and add to it inside. A name that collects as the loop goes is called an **accumulator**, or a running total. It must be made before the loop: made inside, it would start again at 0 every round.

```lua
local minutes = 0
for lap = 1, 4 do
  minutes = minutes + 5      -- every lap takes 5 minutes
end
return minutes               -- 20
```

**for or while.** Use `for` when you know how many rounds there are, and `while` when you only know when to stop. A `for` loop cannot forget to move its counter on, so it rules out a whole kind of infinite loop.

**Other languages.** Rust writes `for day in 1..=30 { }` and Java `for (int day = 1; day <= 30; day++) { }`. The spelling differs, but every version says the same three things: where to start, where to stop and what to do each round.

## Your turn

- In `pushups_after`, make a running total that starts at 0.
- Loop `day` from 1 to `days` and add each day's push-ups (that is `day` itself).
- Return the total. With 0 days, the loop runs zero times.
