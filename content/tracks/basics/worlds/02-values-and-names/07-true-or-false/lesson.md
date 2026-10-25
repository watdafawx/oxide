The machine has to answer yes-or-no questions: is the show sold out? Is this person old enough for the late show? Those answers are a third kind of value.

**Yes or no.** `true` and `false` are values of their own type, called **boolean** (after George Boole, who worked out the maths of yes and no). They are written without quotes; `"true"` in quotes would be a string.

**Comparing makes one.** A comparison works out to a boolean: `age >= 15` is `true` when age is 15 or more. The signs are `<` less than, `>` greater than, `<=` and `>=` for "or equal", `==` equal, and `~=` not equal.

```lua
function M.has_room(free, wanted)
  return free >= wanted      -- true when there are enough seats
end
```

**Two equals signs.** `==` asks "are these equal?"; a single `=` stores a value. They are easy to mix up, and Lua refuses `if age = 15` with an error instead of guessing. Most other languages write "not equal" as `!=`; Lua writes `~=`.

**Why a type for it.** A boolean is the smallest answer there is, and the next world is built on it: every decision a program makes comes down to one `true` or `false`.

## Your turn

- In `sold_out`, return whether `free` is exactly 0.
- In `old_enough`, return whether `age` is 15 or more.
- Save. The tests check the edges: 0 and 1 seats, ages 14 and 15.
