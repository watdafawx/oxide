You cannot bake 3 batches with 2 eggs. The calculator should refuse and say why, and the code that called it should be able to carry on instead of crashing.

**Stop with a message.** `error("the jug is empty")` stops the function on the spot and raises that message. Nothing after it runs.

**Nobody catches it.** Left alone, an error travels up and stops the whole program, or fails the test, and prints the message.

**Catch it.** `pcall(fn, a, b)` calls `fn(a, b)` in a protected way. It never stops: it always comes back with two things, and you catch them in two names.

```lua
function M.pour(milk)
  if milk == 0 then
    error("the jug is empty")
  end
  return milk - 1
end

local ok, result = pcall(M.pour, 0)
-- ok is false, result is the message
local fine, left = pcall(M.pour, 5)
-- fine is true, left is 4
```

**Two outcomes.** When the function worked, `ok` is `true` and `result` is what it returned. When it raised, `ok` is `false` and `result` is the message.

**A prefix.** Lua puts `file:line:` in front of the message. Write `error("...", 0)` to leave that out.

**If this is new.** Programming 101 explains error messages from zero: [Read the red](/quest/basics/01-what-a-program-is/05-read-the-red).

## Your turn

- In `bake`, raise `"not enough eggs"` when `eggs` is less than `needed`.
- Otherwise return what is left: `eggs - needed`.
- In `try_bake`, return `pcall(M.bake, eggs, needed)`.
