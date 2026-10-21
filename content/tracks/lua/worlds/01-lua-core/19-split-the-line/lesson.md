A whole recipe arrives as one line: `3 eggs,2 flour`. The last quest read one ingredient. Now the calculator has to cut the line into ingredients first.

**A line of pieces.** The commas are the cuts. What lies between them is one ingredient each: `3 eggs`, then `2 flour`.

**One match at a time.** `string.gmatch(text, pattern)` finds every piece of `text` that fits the pattern, one per round of a `for` loop. The loop variable is the piece itself.

**Not a comma.** `[^,]` is one character that is *not* a comma: the `[ ]` list characters and the `^` inside means "anything except". With `+` after it, `[^,]+` is a run of non-commas, which is exactly one piece.

```lua
for piece in string.gmatch("3 eggs,2 flour", "[^,]+") do
  -- piece is "3 eggs", then "2 flour"
end
```

**Collect them.** Make an empty list before the loop and `table.insert` each piece into it, then return the list.

**Use last quest's work.** The starter already holds `parse_ingredient` from the last quest. `parse_recipe` calls it on each piece and stores the result as a table with a `count` and a `name`.

## Your turn

- In `split_commas`, loop with `string.gmatch` and collect each piece into a list.
- In `parse_recipe`, call `M.parse_ingredient` on every piece.
- Insert `{count = count, name = name}` for each, and return the list.
