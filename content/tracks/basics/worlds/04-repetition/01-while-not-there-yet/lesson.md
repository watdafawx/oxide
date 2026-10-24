Your training plan starts at 2 km and adds 1.5 km every week. How many weeks until you can run 10 km? You could work it out on paper, but the plan should answer for any goal, and that means doing one step again and again until the goal is reached.

**Again and again.** Say it in words: while the distance is below the goal, add a week and run 1.5 km further. Then ask again. Writing those lines out one after another would mean knowing in advance how many weeks it takes, which is the very thing you are asking.

**While.** `while condition do ... end` runs the lines inside, checks the condition again, and keeps going for as long as it is true. Code that repeats like this is a **loop**, and each pass through it is one **iteration**.

```lua
local cups = 0
local water = 0
while water < 2000 do      -- 2000 ml of water a day
  water = water + 250
  cups = cups + 1
end
return cups                 -- 8 cups
```

**Something must change.** The loop only ends because `water` grows on every pass. If nothing inside changes the condition, it stays true forever: an **infinite loop**. Oxide stops a run after 5 seconds; a real program would just hang.

**Maybe zero times.** The condition is checked before the first pass too. If it is false from the start, the inside never runs at all, and that is correct: if 2 km already reaches the goal, the answer is 0 weeks.

## Your turn

- In `weeks_until`, start the distance at 2 and the weeks at 0.
- Loop while the distance is below `goal`: add 1.5 km and one week each time.
- Return the weeks once the loop is done.
