Before crafting, the calculator asks yes-or-no questions: are there enough plates? Is the stack full? Each question has a one-word answer.

**Ask a question.** `>=` means "at least". `plates >= 2` asks "do I have 2 plates or more?". `>` and `<` ask "more than" and "less than".

**True or false.** The answer to a question is a value too: `true` or `false`. A value that is one of those two is called a **boolean**.

**Equal, and not equal.** `==` asks "is it the same?". Use two equals signs: a single `=` would give a name to something instead of asking. `~=` asks "is it different?".

```lua
local enough = plates >= 2   -- true when plates is 2 or more
local full = count == 100    -- true when it is exactly 100
local partial = count ~= 100 -- true when it is anything else
```

**Hand the answer back.** A question is a value, so a function can return it directly: `return plates >= 2`. No extra steps needed.

## Your turn

- In `can_craft_gear`, return whether `plates` is at least 2.
- In `stack_is_full`, return whether `count` is exactly 100.
- In `stack_is_partial`, return whether `count` is not 100.
