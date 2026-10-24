Your recipe calculator starts with one question: how many eggs do four batches of pancakes need? Before any maths, the code needs a way to say "the number of batches".

**Name a number.** You plan 4 batches. In Lua you write that as `local batches = 4` and read it as "local *batches* is 4". From that line on, the word `batches` means 4.

**The word for it.** A name like this is called a **variable**: a name for a value. The word `local` keeps the name private to the function you are in. Always write it.

**Maths with names.** `*` multiplies, `+` adds and `-` takes away. One batch takes 3 eggs, so `batches * 3` is 12. Give that a name too:

```lua
local batches = 4
local eggs = batches * 3   -- 12. Two dashes start a note for humans.
```

**Write it once.** The point of a name is that the number lives in **one** place. Tomorrow you want 6 batches: change the `4` and `eggs` is right everywhere.

**Already written.** The last line of each starter function, `return eggs`, hands the value back. Leave it alone for now.

**If this is new.** Programming 101 explains variables from zero: [A name for a value](/quest/basics/02-values-and-names/02-a-name-for-a-value).

## Your turn

- In `planned_batches`, change the `local` line so `batches` is 4.
- In `eggs_needed`, make `eggs` equal `batches * 3`.
- Save. Both objectives turn green.
