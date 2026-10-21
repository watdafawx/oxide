A long recipe book would use a lot of memory if the calculator built every scaled recipe up front. A function that hands out one value, pauses, and carries on only when asked is a better fit. Lua calls that a coroutine.

**Pause in the middle.** `coroutine.yield(x)` pauses the running function and hands out `x`. The function does not finish. It just waits where it is.

**Wrap it.** `coroutine.wrap(fn)` turns a function that pauses into an ordinary function you can call. Each call runs it until the next `yield` and gives you the value that was handed out.

```lua
function M.names_of(list)
  return coroutine.wrap(function()
    for _, name in ipairs(list) do
      coroutine.yield(name)       -- pause here, hand out one name
    end
  end)
end

for name in M.names_of({"flour", "eggs"}) do
  -- name is "flour", then "eggs"
end
```

**A generator.** A `for ... in` loop can call such a function for you, once per round, until it finishes. A function built this way is called a **generator**.

**Pauses are real.** Code after a `yield` does not run until you ask for the next value. When the function runs out of things to give, the loop ends.

**Where you will use it.** The boss at the end of this world reads a formula one token at a time, and that tokenizer is exactly a generator.

## Your turn

- In `scaled`, return `coroutine.wrap` around a function.
- Inside it, loop over `counts` with `ipairs`.
- `yield` each count multiplied by `factor`.
