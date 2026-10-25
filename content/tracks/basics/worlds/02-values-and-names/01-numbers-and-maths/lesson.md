The cinema's ticket machine has to work out prices. A ticket costs 9 and a popcorn costs 4, so the first thing the machine needs is plain arithmetic.

**Numbers are values.** A **value** is one piece of data a program works with. `9` is a value, and so is the text `"Open 9 to 5"` from the last world. Numbers are written without quotes, and decimals use a dot: `8.5`.

**The four signs.** `+` adds, `-` subtracts, `*` multiplies and `/` divides. Keyboards have no × or ÷ keys, which is why nearly every programming language uses `*` and `/`.

**Order.** As in school maths, `*` and `/` happen before `+` and `-`, so `2 + 3 * 4` is 14, not 20. Brackets go first: `(2 + 3) * 4` is 20. When in doubt, add brackets; they cost nothing and show the next reader the order you meant.

```lua
function M.couple_total()
  return 2 * 9 + 4      -- two tickets and one popcorn: 22
end
```

**Expressions.** A piece of code that works out to a value, like `2 * 9 + 4`, is an **expression**. Lua works the expression out first, and then `return` hands back the result: here 22, not the text of the sum.

## Your turn

- In `group_total`, return the price of 4 tickets at 9 plus one popcorn at 4.
- In `price_each`, return that same total split evenly between the 4 friends.
- Save and check both objectives.
