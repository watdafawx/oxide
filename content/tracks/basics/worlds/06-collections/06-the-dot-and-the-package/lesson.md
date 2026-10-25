Shop staff add new products all the time, change prices and take old products off the list. Changing a map works like changing a list slot, and once you know it, the `M` at the top of every starter stops being a mystery.

**Add or change.** `prices["eggs"] = 3` adds the key `eggs` if it is new, or replaces its value if it is already there. `prices["eggs"] = nil` removes the key again, because a key holding nothing is a key that is not there.

**The dot.** When a key is a plain name, `prices.eggs` is shorthand for `prices["eggs"]`; the two mean exactly the same. The square brackets are needed when the key is in a variable (`prices[item]`) or contains a space or a dash.

```lua
local stock = {}
stock.apples = 12           -- the same as stock["apples"] = 12
stock["green tea"] = 4      -- a space: brackets needed
```

**The package, opened.** `local M = {}` makes an empty table, and `function M.price_of(...)` stores a function in it under the key `price_of`. `return M` hands that table to whoever asked for the file, which is the test. A file of functions packed into a table like this is called a **module**.

**Any kind of value.** A table's values can be numbers, text, booleans, functions or other tables. In Lua the table is the one tool for lists, maps and modules; other languages have separate tools for each, like Java's `ArrayList` and `HashMap`, or Rust's `Vec` and `HashMap`.

## Your turn

- In `set_price`, store `price` under `item` in `prices`.
- In `discontinue`, remove `item` from `prices`.
- Neither function returns anything: the test looks at the table it handed in.
