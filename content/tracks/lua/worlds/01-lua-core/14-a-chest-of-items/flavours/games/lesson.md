A chest is not a list of positions. It is "coal: 12, iron-plate: 3", where you ask by item name and get a count back. Lua tables can do that too, and Factorio's data is full of them.

**Names as keys.** `{coal = 12, stone = 3}` is a table where each value has a name, called a **key**. A table used this way is a **dictionary**, or *dict*.

**Read and write.** `chest.coal` reads the value under the key `coal`. `chest.coal = 10` changes it, and writing a key that is not there yet adds it.

**Keys in a variable.** When the key is in a name, put the name in square brackets: with `item = "stone"`, `chest[item]` is the stone count. A key with a dash needs brackets and quotes: `chest["iron-plate"]`, and the same inside `{ }`: `{["iron-plate"] = 3}`.

**Missing is nil.** Reading a key that is not there gives `nil`, not an error. Together with `or`, `chest[item] or 0` means "the count, or 0 when there is none".

**Delete a key.** Set it to `nil` and the key is gone: `chest.coal = nil`.

```lua
local chest = {coal = 12, ["iron-plate"] = 3}
local n = chest.coal                   -- 12
chest["copper-plate"] = 5              -- a new key
local item = "iron-plate"
local plates = chest[item]             -- 3
local nothing = chest.stone            -- nil: no such key
chest.coal = nil                       -- the key is gone
```

## Your turn

- In `count_of`, return the item's count, or 0 when it is missing.
- In `put_in`, add `n` to the item's count.
- In `empty_out`, remove the item and return the count it had.
