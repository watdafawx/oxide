A pantry is not a list of positions. It is "eggs: 12, flour: 3", where you ask by name and get a count back. Lua tables can do that too.

**Names as keys.** `{eggs = 12, flour = 3}` is a table where each value has a name, called a **key**. A table used this way is a **dictionary**, or *dict*.

**Read and write.** `pantry.eggs` reads the value under the key `eggs`. `pantry.eggs = 10` changes it, and writing a key that is not there yet adds it.

**Keys in a variable.** When the key is in a name, put the name in square brackets: with `item = "flour"`, `pantry[item]` is the flour count. A key with a dash needs brackets and quotes: `pantry["brown-sugar"]`.

**Missing is nil.** Reading a key that is not there gives `nil`, not an error. Together with `or`, `pantry[item] or 0` means "the count, or 0 when there is none".

**Delete a key.** Set it to `nil` and the key is gone: `pantry.eggs = nil`.

```lua
local pantry = {eggs = 12, flour = 3}
local n = pantry.eggs              -- 12
pantry.sugar = 5                   -- a new key
local item = "flour"
local flour = pantry[item]         -- 3
local nothing = pantry.salt        -- nil: no such key
pantry.eggs = nil                  -- the key is gone
```

**If this is new.** Programming 101 explains looking a value up by name from zero: [Look it up](/quest/basics/06-collections/05-look-it-up).

## Your turn

- In `count_of`, return the item's count, or 0 when it is missing.
- In `put_in`, add `n` to the item's count.
- In `empty_out`, remove the item and return the count it had.
