A ticket has to be printed: the film, the seat, the price. All of that is text, and text is a different kind of value from a number.

**Text values.** Text in quotes is called a **string**, short for "a string of characters": letters, digits, spaces and symbols in a row. `"Dune"` is a string. Lua also accepts single quotes, `'Dune'`, which helps when the text itself contains a `"`.

**Joining.** `..` (two dots) glues two strings into one: `"Screen " .. "3"` is `"Screen 3"`. This is called **concatenation**. No spaces are added for you, so put them inside the quotes where you want them.

**Numbers join too.** Gluing a number with `..` turns it into text first: `"Row " .. 12` is `"Row 12"`. That is handy for tickets. It is also a first sign that Lua converts between kinds of values when it thinks it knows what you mean.

```lua
function M.screen_line(film, screen)
  return film .. " on screen " .. screen     -- "Dune on screen 3"
end
```

**Why `..` and not `+`.** Java and JavaScript glue text with `+`. Lua keeps `+` for numbers only, so `"3" .. "4"` is `"34"` while `3 + 4` is 7, and nobody has to guess which one you meant.

## Your turn

- In `seat_line`, glue `film` and `seat` into text like `Up, seat 14` (mind the comma and the spaces).
- In `price_line`, build text like `Price: 9 euros` around `price`.
- Save. The tests try other films and prices too.
