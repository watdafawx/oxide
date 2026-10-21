The calculator prints a shopping list. A line like `3 eggs` is text built from a number and a word, and the code has to glue the pieces together.

**Text in quotes.** `"eggs"` in quotes is text, called a **string**. The quotes tell Lua "this is words, not a name".

**Glue.** `..` (two dots) glues two strings into one: `"egg" .. "s"` is `"eggs"`. Chain as many pieces as you like.

**Numbers glue too.** A number next to `..` turns into its text on the way, so `count .. " eggs"` works when `count` is 3. Put a space on each side of `..` so the dots are not read as part of a number.

**Spaces are pieces.** `..` adds nothing between its two sides. `count .. item` gives `"3eggs"`. If you want a space, glue a `" "` in too.

```lua
local count = 3
local item = "eggs"
local line = count .. " " .. item    -- "3 eggs"
```

## Your turn

- In `ingredient_label`, return the count, a space and the item, glued with `..`.
- In `cooking_line`, return the dish, `": "`, the minutes and `" min"`.
- Save. Both objectives turn green.
