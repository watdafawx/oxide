Recipes come from text files, one line per ingredient: `3 eggs`. The calculator cannot do maths on a line of text. It has to pull the number and the word out of it.

**Text, not data.** `"3 eggs"` is one string. The `3` in it is a character, not a number, and no maths works on it yet.

**Describe the shape.** `string.match(text, pattern)` looks for the first piece of `text` that fits a **pattern**, a description of a shape. `%d` is one digit and `+` after it means "one or more", so `%d+` is a whole number written in text. `%S+` is a whole word: `%S` is any character that is not a space.

**Catch the pieces.** Round brackets in the pattern mark the parts to hand back. A call can give back several values, and `local count, name = ...` catches them.

```lua
local count, name = string.match("3 eggs", "(%d+) (%S+)")
-- count is "3" (still text), name is "eggs"
local n = tonumber(count)    -- 3, a real number now
```

**Text to number.** `tonumber("3")` turns text into a number you can calculate with. `return a, b` hands back two values the same way.

**No fit, no value.** If the text does not fit the pattern, `match` gives back `nil`, and `tonumber(nil)` is `nil` too.

## Your turn

- In `parse_ingredient`, use `string.match` with the pattern from the lesson.
- Return the count as a number and the name.
- Check that `"eggs"`, with no number, gives `nil, nil`.
