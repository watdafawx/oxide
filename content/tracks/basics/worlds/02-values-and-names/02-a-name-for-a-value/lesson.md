The ticket price shows up everywhere in the machine: single tickets, groups, late shows. Typing `9` in every one of those places means hunting down every one of them the day the price goes up.

**Give it a name.** `local price = 9` makes a name, `price`, and stores 9 under it. From that line on, writing `price` means 9. Read the line as "store 9 in price".

**The word for it.** A name that holds a value is called a **variable**. Picture a labelled box: `price` is the label and 9 is what is inside. Using the name looks inside the box and takes out what is there.

```lua
local price = 9
local snack = 3
return price * 2 + snack     -- 21
```

**Why `local`.** `local` means the name only exists inside the part of the code where it was made, here the function. Always write it when you make a new name; world 5 shows what goes wrong without it.

**Good names.** Names may use letters, digits and `_`, but cannot start with a digit or contain spaces or dashes (`ticket-price` would mean "ticket minus price"). So programmers write `ticket_price`. Pick names that say what the value means: `price` beats `p`, and the next reader, often you, will thank you.

**One place.** With a name, the price lives on one line. When it changes, you edit that line, and every sum that uses `price` follows along.

## Your turn

- In `order_total`, give `price` and `popcorn` their real values (9 and 4).
- Return the cost of 3 tickets and 2 popcorns, written with the names, not the numbers.
- `late_show_total` is a different order at the late-show price of 7: fix its price line only.
