The league table's line for the Owls says 9 points after 2 wins and 1 draw, and it should say 7. The games count is wrong as well. The code is short, but staring at it harder will not find the bugs. Narrowing down will.

**Don't stare.** Rereading code in search of a bug mostly shows you what you meant to write, not what you wrote. Debugging works better as a search: split the code into parts, and find out which part is wrong.

**Ask the pieces.** Here the table line is built from two smaller functions, and each one has its own test. If `points` is red and `played` is green, the bug is inside `points`: the tests have already narrowed the search for you.

**Print the middle.** Inside a function, print the values between steps: `print("total", total, "count", count)` (print takes several values, separated by commas). When a test fails, Oxide shows what your code printed under the objective's **why?**.

```lua
function M.average(total, count)
  print("total", total, "count", count)   -- look inside
  return total / count
end
```

**Compare with paper.** Work out on paper what each value *should* be for the test's input: 2 wins and 1 draw make 6 points from wins and 1 from the draw. The first printed value that differs from your paper is where the bug is.

**One change at a time.** Fix one thing, save, and look again. If you change three things and it turns green, you do not know which one mattered; if it gets worse, you do not know which one broke it. Remove the prints when you are done.

## Your turn

- Save and see which objectives are red. Start with `points`.
- Print the values inside it, compare them with your paper sum, and fix the line that is wrong.
- Then do the same for `played`. Watch what happens to `table_line`.
