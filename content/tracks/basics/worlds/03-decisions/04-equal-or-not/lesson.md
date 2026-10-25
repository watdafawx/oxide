The staff door at the back of the park opens only for the code 4721. Checking whether two values are the same is the most common question code asks, and it has a trap built in.

**Try the obvious thing.** The starter says `if code = 4721 then`, the way you might write it on paper. Save, and Lua refuses to run the file at all:

```text
src/staff_door.lua:5: 'then' expected near '='
```

**Why it refuses.** A single `=` means "store", and storing is not a question. Lua expected the condition to finish and `then` to follow, found `=` instead, and says so. This is a mistake in the grammar of the code, a **syntax error**, and it is the one kind Lua finds while reading the file, before any line runs.

**Ask, don't store.** `==` asks "are these equal?" and gives true or false. `~=` asks "are these different?". So `code == 4721` is the question the door needs, and `code ~= 4721` is its opposite.

**Already a boolean.** A comparison already is true or false. So instead of `if ... then return true else return false end`, a function can simply return the comparison. Both are correct; the short one says the same thing in one line.

**Other languages.** Java and Rust also use `==` for "equal" and write "not equal" as `!=`. They refuse `if (code = 4721)` too, for the same reason.

## Your turn

- Save once and read the message.
- Fix the condition in `door_opens` so it asks instead of stores.
- In `sound_alarm`, return true for any code that is not 4721.
