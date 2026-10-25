The machine's code is getting longer, and next month someone else will change it. Notes inside the code tell them why it does what it does.

**Two dashes.** Everything after `--` on a line is a **comment**: a note for people. Lua skips it completely. You have seen them in every starter, from `-- TODO` onwards.

**Switching a line off.** Because comments are skipped, putting `--` in front of a line of code switches it off without deleting it. Programmers call this **commenting out**. It is handy for trying something and going back.

```lua
local price = 9
-- price = 7    -- the summer price, switched off until June
```

**Longer notes.** `--[[` starts a comment that runs over several lines, up to `]]`. Rust and Java use `//` for one line and `/* ... */` for several; the idea is the same.

**What to write.** A good comment says **why**, because the code already says what. `-- staff get 2 off` beats `-- subtract 2`. A wrong comment is worse than none, so change it whenever you change the code.

## Your turn

- In `staff_price`, switch the wrong line off and the staff discount on.
- In `matinee_price`, switch on the line inside the long comment by removing the comment's start and end.
- Save. Both objectives turn green.
