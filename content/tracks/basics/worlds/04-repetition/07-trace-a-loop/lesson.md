Loops are where code gets hard to picture: the same lines run again and again, with different values each time. The tool every programmer uses for that is a pencil and a small table.

**Make a table.** Write one column per variable and one row per round. Fill in each row by following the code exactly, line by line, using the values from the row above.

```lua
local total = 0
for day = 1, 4 do
  total = total + day * 2
end
```

```text
day | total
 -  |   0      before the loop
 1  |   2      0 + 1 * 2
 2  |   6      2 + 2 * 2
 3  |  12      6 + 3 * 2
 4  |  20     12 + 4 * 2
```

**Read it off.** After the loop, `total` is whatever the last row says: 20. If the program gives something else when it runs, the table shows you exactly which round went wrong.

**Check the edges.** Trace the first round, the last round, and the case where the loop runs zero times. Those three find most loop bugs. The classic is the **off-by-one error**, where a loop runs one round too many or one too few.

**Let the computer trace.** A `print(day, total)` inside the loop prints one row per round. When a test fails, Oxide shows what your code printed, and world 8 uses that to hunt down bugs.

## Your turn

Answer the quiz, then start the next quest.
