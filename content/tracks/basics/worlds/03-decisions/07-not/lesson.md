The gate also needs the opposite questions: is the park *not* full? Is the ride *not* safe to open? Flipping a yes into a no is the last piece of logic, and then the gate can say anything.

**Flip it.** `not` turns true into false and false into true: `not full` is true when `full` is false. It goes in front of the value it flips.

```lua
local raining = true
local go_outside = not raining      -- false
local open = not (raining and cold) -- brackets: flip the whole join
```

**When to use it.** Use `not` when the name you have says the opposite of what you are asking. `if not sold_out then` reads better than `if sold_out == false then`, and it means the same thing.

**Flipping a join.** `not (a and b)` flips the whole join in the brackets: it is true when at least one of `a` and `b` fails. That is the same as `not a or not b`. These swaps are called **De Morgan's laws**, and they help when you tidy up a long condition.

**Other languages.** Rust and Java write `not` as `!`, `and` as `&&` and `or` as `||`. The ideas are the same; only the symbols differ, so `!full && open` in Java is `not full and open` in Lua.

## Your turn

- In `can_enter`, return true when the park is not `full`.
- In `ride_closed`, return true unless both `staff_ok` and `weather_ok` are true.
- Save. Both objectives turn green.
