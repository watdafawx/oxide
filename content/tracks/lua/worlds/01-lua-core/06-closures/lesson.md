**Why now:** the calculator asks "how much iron does this gear cost?" thousands of times for the same items. A **closure** lets a function carry its own cache with it, and Factorio's event handlers lean on the same trick.

A function remembers the local variables it was created next to (**upvalues**):

```lua
local function make_counter()
  local count = 0              -- lives on after make_counter returns
  return function()
    count = count + 1
    return count
  end
end

local a, b = make_counter(), make_counter()
print(a(), a(), b())           -- 1  2  1   (separate counts)
```

- Each call to `make_counter` makes a fresh `count`, so counters don't interfere.
- A `for i = 1, 3` loop makes a **new** `i` each round, so `function() return i end` inside it captures that round's value. A `local i` declared *before* a `while` loop is shared by every closure instead.
- **Forgot `local`?** `count = 0` creates a *global* that every function sees. Always write `local`.
- **Memoize** = wrap a function with a cache table so repeat calls are free.

## Your turn

- Open `src/closures.lua`.
- Fill in `make_counter`, `make_multipliers` and `memoize`.
