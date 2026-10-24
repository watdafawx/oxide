Your recipe calculator starts with one question: how many iron plates do five gears cost? Before any maths, the code needs a way to say "the number of gears".

**Name a number.** You want 5 gears. In Lua you write that as `local gears = 5` and read it as "local *gears* is 5". From that line on, the word `gears` means 5.

**The word for it.** A name like this is called a **variable**: a name for a value. The word `local` keeps the name private to the function you are in. Always write it.

**Maths with names.** `*` multiplies, `+` adds and `-` takes away. One gear takes 2 iron plates, so `gears * 2` is 10. Give that a name too:

```lua
local gears = 5
local plates = gears * 2   -- 10. Two dashes start a note for humans.
```

**Write it once.** The point of a name is that the number lives in **one** place. Tomorrow you want 8 gears: change the `5` and `plates` is right everywhere.

**Already written.** The last line of each starter function, `return plates`, hands the value back. Leave it alone for now.

**If this is new.** Programming 101 explains variables from zero: [A name for a value](/quest/basics/02-values-and-names/02-a-name-for-a-value).

## Your turn

- In `gears_wanted`, change the `local` line so `gears` is 5.
- In `plates_needed`, make `plates` equal `gears * 2`.
- Save. Both objectives turn green.
