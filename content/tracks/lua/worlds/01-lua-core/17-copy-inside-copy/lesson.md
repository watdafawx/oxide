A recipe holds its name, its time and a list of ingredients, and the list is a table inside the table. The copy loop from the last quest copies the outer table, and the ingredient list is still shared with the original.

**One level only.** The `pairs` copy hands every value across as it is. When a value is itself a table, the copy gets the *same* inner table, not a new one. Change an ingredient in the copy and the original changes too.

**Ask what it is.** `type(value)` gives the kind of a value as text: `"table"`, `"number"`, `"string"`. So `type(value) == "table"` finds the values that need copying and `type(value) ~= "table"` finds the plain ones.

**A function that calls itself.** A table inside a table is the same problem on a smaller piece, so the function can hand the piece to itself. That is **recursion**. It stops at plain values, which have nothing inside.

```lua
function M.sum_all(value)
  if type(value) ~= "table" then
    return value                       -- a plain number: stop here
  end
  local total = 0
  for _, inner in pairs(value) do
    total = total + M.sum_all(inner)   -- the function calls itself
  end
  return total
end
```

**Same shape for copying.** A **deep copy** has the same shape with a different job: a plain value is returned as it is, and a table gets a new empty table filled with a copy of each inner value.

## Your turn

- In `deep_copy`, return `value` as it is when it is not a table.
- Otherwise build a new table with a `pairs` loop, copying each inner value by calling `M.deep_copy` on it.
- Return the new table.
