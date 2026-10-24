You cannot craft 3 gears with 4 plates. The calculator should refuse and say why, and the code that called it should be able to carry on instead of crashing. Factorio mods lean on this: one bad entity must not take down the whole game.

**Stop with a message.** `error("the boiler is out of coal")` stops the function on the spot and raises that message. Nothing after it runs.

**Nobody catches it.** Left alone, an error travels up and stops the whole program, or fails the test, and prints the message.

**Catch it.** `pcall(fn, a, b)` calls `fn(a, b)` in a protected way. It never stops: it always comes back with two things, and you catch them in two names.

```lua
function M.refuel(coal)
  if coal == 0 then
    error("the boiler is out of coal")
  end
  return coal - 1
end

local ok, result = pcall(M.refuel, 0)
-- ok is false, result is the message
local fine, left = pcall(M.refuel, 5)
-- fine is true, left is 4
```

**Two outcomes.** When the function worked, `ok` is `true` and `result` is what it returned. When it raised, `ok` is `false` and `result` is the message.

**A prefix.** Lua puts `file:line:` in front of the message. Write `error("...", 0)` to leave that out.

**If this is new.** Programming 101 explains error messages from zero: [Read the red](/quest/basics/01-what-a-program-is/05-read-the-red).

## Your turn

- In `craft`, raise `"not enough plates"` when `plates` is less than `needed`.
- Otherwise return what is left: `plates - needed`.
- In `try_craft`, return `pcall(M.craft, plates, needed)`.
