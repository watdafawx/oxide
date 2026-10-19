**Why now:** the boss parses text one token at a time, and the clean way to write that is a **generator**: a function that hands out one value and pauses until you ask for the next. (Note: Factorio itself has **no coroutines**. Here they are a language idea you will meet in other Lua.)

```lua
local function count_to(n)
  return coroutine.wrap(function()
    for i = 1, n do coroutine.yield(i) end   -- pause here, hand out i
  end)
end
for v in count_to(3) do print(v) end         -- 1  2  3

local co = coroutine.create(function(a)
  local b = coroutine.yield(a * 2)           -- pauses; resume(co, x) sets b = x
  return a + b
end)
print(coroutine.resume(co, 5))    -- true  10
print(coroutine.resume(co, 7))    -- true  12
print(coroutine.status(co))       -- dead
```

- `create` + `resume` give you control; `wrap` returns a plain function that resumes for you (and re-raises errors).
- `resume` returns `true, ...` or `false, errmessage`.
- Statuses: `suspended`, `running`, `normal`, `dead`.
- Only the coroutine itself calls `yield`; whoever called `resume` gets the values.

## Your turn

- Open `src/gen.lua`.
- Fill in `range`, `accumulator` and `round_robin`.
