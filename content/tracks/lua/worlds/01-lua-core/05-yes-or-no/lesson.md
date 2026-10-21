Before baking, the calculator asks yes-or-no questions: are there enough eggs? Is the carton full? Each question has a one-word answer.

**Ask a question.** `>=` means "at least". `eggs >= 3` asks "do I have 3 eggs or more?". `>` and `<` ask "more than" and "less than".

**True or false.** The answer to a question is a value too: `true` or `false`. A value that is one of those two is called a **boolean**.

**Equal, and not equal.** `==` asks "is it the same?". Use two equals signs: a single `=` would give a name to something instead of asking. `~=` asks "is it different?".

```lua
local enough = eggs >= 3     -- true when eggs is 3 or more
local full = eggs == 12      -- true when it is exactly 12
local partial = eggs ~= 12   -- true when it is anything else
```

**Hand the answer back.** A question is a value, so a function can return it directly: `return eggs >= 3`. No extra steps needed.

## Your turn

- In `can_make_batch`, return whether `eggs` is at least 3.
- In `carton_is_full`, return whether `eggs` is exactly 12.
- In `carton_is_partial`, return whether `eggs` is not 12.
