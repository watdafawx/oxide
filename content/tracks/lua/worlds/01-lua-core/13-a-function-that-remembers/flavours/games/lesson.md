You want a scaler that doubles a recipe, another that triples it, and a counter for how many crafts an assembler has finished. Each one needs a number it carries around. Lua functions can do that, and Factorio's event handlers rely on it.

**A function that makes functions.** A function can `return` another function. Every call builds a fresh one.

**It remembers.** The inner function keeps using the locals that were around it when it was built, even after the outer function has finished. Those remembered locals are called **upvalues**, and a function that carries them is a **closure**.

```lua
function M.make_labeler(prefix)
  return function(item)
    return prefix .. item
  end
end

local mark = M.make_labeler("raw-")
local name = mark("wood")        -- "raw-wood"
```

**One memory each.** Each call to the outer function makes a new `prefix`. A `mark` and a `tag` built from two calls never share it.

**It can change it.** The inner function can also assign to what it remembers. `count = count + 1` inside it changes the same `count` next time it is called.

## Your turn

- In `make_multiplier`, return a function that multiplies its argument by `factor`.
- In `make_counter`, keep a `local count = 0` and return a function that adds 1 and gives it back.
- Save. Both objectives turn green.
