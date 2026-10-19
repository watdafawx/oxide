**Why now:** a chest is a dict: item name to count. The calculator tallies items, drops empty stacks and clones whole recipe tables before changing them. That is this quest.

```lua
local chest = {iron = 12, copper = 0, coal = 3}
for item, count in pairs(chest) do   -- every key, in NO promised order
  print(item, count)
end
```

- `pairs` visits every key; `ipairs` visits `1, 2, 3...` and stops at the first hole.
- **Order is not yours.** Never rely on the order `pairs` gives you; sort the keys first if you need one.
- **Deleting while looping** is fine for fields that already exist: `chest[item] = nil` inside the loop is allowed. *Adding* new keys mid-loop is undefined.
- `local b = a` copies nothing: both names point at the same table. A *shallow* copy shares nested tables, so changing `b.stats.x` changes `a`'s too. A **deep copy** recurses.

```lua
local function shallow(t)
  local out = {}
  for k, v in pairs(t) do out[k] = v end
  return out
end
```

## Your turn

- Open `src/dicts.lua`.
- Fill in `count_words`, `remove_empty` and `deep_copy`.
