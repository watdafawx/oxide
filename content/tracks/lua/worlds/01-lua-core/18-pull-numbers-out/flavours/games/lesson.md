Recipes come from text, one line per ingredient: `2 iron-plate`. The calculator cannot do maths on a line of text. It has to pull the number and the item name out of it.

**Text, not data.** `"2 iron-plate"` is one string. The `2` in it is a character, not a number, and no maths works on it yet.

**Describe the shape.** `string.match(text, pattern)` looks for the first piece of `text` that fits a **pattern**, a description of a shape. `%d` is one digit and `+` after it means "one or more", so `%d+` is a whole number written in text. `%S+` is a whole word: `%S` is any character that is not a space.

**Catch the pieces.** Round brackets in the pattern mark the parts to hand back. A call can give back several values, and `local count, name = ...` catches them.

```lua
local count, name = string.match("2 iron-plate", "(%d+) (%S+)")
-- count is "2" (still text), name is "iron-plate"
local n = tonumber(count)    -- 2, a real number now
```

**Text to number.** `tonumber("2")` turns text into a number you can calculate with. `return a, b` hands back two values the same way.

**No fit, no value.** If the text does not fit the pattern, `match` gives back `nil`, and `tonumber(nil)` is `nil` too.

## Your turn

- In `parse_ingredient`, use `string.match` with the pattern from the lesson.
- Return the count as a number and the name.
- Check that `"stone"`, with no number, gives `nil, nil`.
