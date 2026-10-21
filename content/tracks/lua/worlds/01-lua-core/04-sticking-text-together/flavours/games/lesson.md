The calculator prints a tooltip for each recipe. A line like `2x iron-plate` is text built from a number and an item name, and the code has to glue the pieces together.

**Text in quotes.** `"iron-plate"` in quotes is text, called a **string**. The quotes tell Lua "this is words, not a name". Factorio item names are strings.

**Glue.** `..` (two dots) glues two strings into one: `"iron" .. "-plate"` is `"iron-plate"`. Chain as many pieces as you like.

**Numbers glue too.** A number next to `..` turns into its text on the way, so `count .. " plates"` works when `count` is 2. Put a space on each side of `..` so the dots are not read as part of a number.

**Spaces are pieces.** `..` adds nothing between its two sides. `count .. item` gives `"2iron-plate"`. If you want an `x ` there, glue it in too.

```lua
local count = 2
local item = "iron-plate"
local line = count .. "x " .. item    -- "2x iron-plate"
```

## Your turn

- In `ingredient_label`, return the count, `"x "` and the item, glued with `..`.
- In `crafting_line`, return the item, `": "`, the seconds and `"s"`.
- Save. Both objectives turn green.
