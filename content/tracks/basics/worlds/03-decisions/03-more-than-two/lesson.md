Park tickets have three prices: children under 12 pay 5, visitors aged 65 or older pay 6, and everyone else pays 10. Two branches are not enough for that.

**Another test.** `elseif` adds a branch with a condition of its own: `if a then ... elseif b then ... else ... end`. Lua checks the conditions from the top down and runs the first branch whose condition is true. The `else`, if there is one, catches the rest.

```lua
if speed > 100 then
  return "fast"
elseif speed > 50 then
  return "medium"
else
  return "slow"
end
```

**Order matters.** Only the first true branch runs. With 120, `speed > 100` is true, so "medium" is never reached, even though `120 > 50` is true as well. Put the narrowest test first; a wide test at the top swallows everything below it.

**One word.** `elseif` is one word in Lua. Writing `else if` with a space starts a second `if` inside the `else`, which then needs an `end` of its own. Rust and Java write `else if` with a space, and there it means what Lua's `elseif` means.

**Cover every case.** List the cases on paper first: under 12, 12 to 64, 65 and up. Every age should land in exactly one of them. Then check the edges: 11, 12, 64 and 65.

## Your turn

- In `ticket_price`, give children under 12 the price 5.
- Give visitors aged 65 and up the price 6.
- Everyone else pays 10. Save and check both objectives.
