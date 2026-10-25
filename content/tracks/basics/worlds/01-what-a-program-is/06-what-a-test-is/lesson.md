Every quest so far turned green on its own when you saved. That was not magic: another small program, a test, ran yours and checked the result. Today you read one, because a test is the clearest description of what a piece of code must do.

**A function.** The screen's lines now live in **functions**: named blocks of steps that run when something asks for them. `function M.opening_hours()` starts one and `end` closes it. For now the starter writes those lines for you; world 5 shows how to write your own.

**Handing back.** `return` ends the function and **hands a value back** to whoever asked for it. Printing shows text to a person; returning gives a value to the code that asked, and that code can then check it.

```lua
local M = {}

function M.motto()
  return "Ride more, worry less"   -- this text goes back to the caller
end

return M
```

**The packaging.** The first and last lines, `local M = {}` and `return M`, package the file's functions so the test can reach them. Leave them alone; world 6 opens the package and shows what is inside.

**A test.** A **test** is a short program that runs your function and compares what comes back with what it should be. `t.eq(sign.motto(), "Ride more, worry less")` reads "the motto should equal this text". When the two differ, the test fails and names both, like `expected "Ride more, worry less", got nil`.

**Why tests exist.** A test writes down what "working" means once, so checking it costs nothing: it runs every time you save. Real projects keep thousands of them, so a change that breaks something old turns red straight away. In Oxide every objective is one test, and green means the job is done.

## Your turn

- Open `tests.lua`. You cannot change it, but you can read it: find what each function must give back.
- In `opening_hours`, return that text instead of `nil`.
- Do the same in `closed_on`, then save.
