Your shopping list should show the biggest buys first, but only you know what "first" means. In Lua you can hand a function a rule written right there, because a function is a value like any number.

**A function is a value.** `function(a, b) return a < b end` is a function with no name. You can pass it around, store it in a name, or give it to another function.

**Hand one to sort.** `table.sort(list, rule)` puts the list in order. For each pair of items it asks your rule "should `a` come before `b`?" and the rule answers `true` or `false`.

```lua
local counts = {5, 1, 3}
table.sort(counts, function(a, b)
  return a < b
end)                              -- counts is now 1, 3, 5
```

**Your functions can take one.** A parameter can hold a function. Call it like any other: `fn(item)` runs the function you were given on that item.

**Build a new list.** To apply a function to every item, make an empty `{}`, loop with `ipairs`, and `table.insert` each answer into it. The original list stays as it was.

## Your turn

- In `sort_biggest_first`, sort `counts` with a rule that puts the bigger count first.
- In `apply_to_each`, build and return a new list holding `fn(item)` for each item.
- Save. Both objectives turn green.
