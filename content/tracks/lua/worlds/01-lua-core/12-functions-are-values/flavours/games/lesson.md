The chest summary should show the biggest stacks first, but only you know what "first" means. In Lua you can hand a function a rule written right there, because a function is a value like any number. Factorio's event handlers are built this way.

**A function is a value.** `function(a, b) return a < b end` is a function with no name. You can pass it around, store it in a name, or give it to another function.

**Hand one to sort.** `table.sort(list, rule)` puts the list in order. For each pair of items it asks your rule "should `a` come before `b`?" and the rule answers `true` or `false`.

```lua
local stacks = {50, 10, 30}
table.sort(stacks, function(a, b)
  return a < b
end)                              -- stacks is now 10, 30, 50
```

**Your functions can take one.** A parameter can hold a function. Call it like any other: `fn(item)` runs the function you were given on that item.

**Build a new list.** To apply a function to every item, make an empty `{}`, loop with `ipairs`, and `table.insert` each answer into it. The original list stays as it was.

## Your turn

- In `sort_biggest_first`, sort `stacks` with a rule that puts the bigger stack first.
- In `apply_to_each`, build and return a new list holding `fn(item)` for each item.
- Save. Both objectives turn green.
