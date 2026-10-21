The chest is a dict, and the calculator needs to total everything in it and sweep out the stacks that hit zero. That means visiting every key, and a dict has no positions to count through.

**Visit every key.** `for item, count in pairs(chest) do ... end` runs once per key. Each round gives you the key (`item`) and its value (`count`).

**No promised order.** `pairs` visits the keys in whatever order Lua likes, and the order can change between runs. Never write code that depends on it.

**Change while looping.** Changing a key that already exists is safe inside the loop, even setting it to `nil`. Adding brand-new keys inside the loop is not.

```lua
for item, count in pairs(chest) do
  if count > 100 then
    chest[item] = 100       -- changing an existing key is safe
  end
end
```

**Lists and dicts.** Use `ipairs` for a list: it walks 1, 2, 3 in order. Use `pairs` for a dict: it gives every key, in any order.

## Your turn

- In `total_items`, loop with `pairs` and add up every count.
- In `remove_empty`, set the key to `nil` for every count that is 0.
- Return the chest itself from `remove_empty`.
