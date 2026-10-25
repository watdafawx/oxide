The painter shows a star next to the title when the picture has unsaved changes. It compares the picture with the last saved copy, and even when the two look exactly alike, `==` says they differ.

**The same table.** For tables, `==` asks "are these the same table?", not "do these hold the same things?". Two tables made separately are never `==`, even with identical items: `{1, 2} == {1, 2}` is false.

**Why.** Comparing references is instant: are the two addresses the same? Comparing contents means visiting every item, which is slow for big tables and unclear for tables inside tables. So Lua does the quick, exact thing and leaves the rest to you.

```lua
local a = {"red"}
local b = a
local c = {"red"}
-- a == b is true:  one table, two names
-- a == c is false: two tables that look alike
```

**Compare contents yourself.** To ask "same items?", check the lengths first, then each slot in a loop. The first difference means no, and you can return straight away. If the loop gets to the end without finding one, they match.

**Other languages.** Java has the same split: `==` means "the same object" and `.equals(...)` means "the same contents", and mixing them up is a classic Java bug. In Rust, `==` compares contents.

## Your turn

- In `same_colours`, give back false when the two lists have different lengths.
- Loop over the slots and give back false at the first colour that differs.
- If nothing differed, give back true.
