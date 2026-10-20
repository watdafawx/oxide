**Why now:** the Kit Mod should keep inventories tidy: its parts get their own subgroup, and big stacks get smaller. But other mods add items too, and you only see theirs if you run **late enough**.

The game runs a round at a time over **every** mod:

```text
data.lua            of mod A, then mod B, ...     define your own prototypes
data-updates.lua    of mod A, then mod B, ...     tweak what others defined in data.lua
data-final-fixes.lua  of mod A, then mod B, ...   last word: balance, cleanup, bulk edits
```

Inside `data.lua` a later mod's items do not exist yet. In `data-updates.lua` every mod's `data.lua` has run. In `data-final-fixes.lua` every mod's `data-updates.lua` has run too.

```lua
-- data-final-fixes.lua: sees EVERY item from EVERY mod
for _, item in pairs(data.raw.item) do
  if item.stack_size > 50 then item.stack_size = math.floor(item.stack_size / 2) end
end
```

- `/` is float division: `75 / 2` is `37.5`, and a stack size must be a whole number, hence `math.floor`. Real Factorio rejects the fraction.
- Each stage file runs **once**. Do a change in one stage, not several; a halving run twice quarters the stack.
- `pairs` order is not guaranteed. Edit every item the same way and the order does not matter.
- The fake `other` mod next to `mod/` plays "some other mod" in this quest. Do not edit it.

## Your turn

- `mod/data.lua`: already defines the `kit-parts` subgroup.
- `mod/data-updates.lua`: move `other-widget` into `kit-parts`.
- `mod/data-final-fixes.lua`: halve every item stack above 50.
