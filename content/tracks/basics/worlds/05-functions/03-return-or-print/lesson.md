A colleague wrote `delivery_fee` and swears it works: the right number shows up on screen. Yet the order total that uses it fails every time. The cause is the oldest beginner confusion there is.

**It looks right.** The starter's `delivery_fee` ends with `print(fee)`. Save, tap **why?** on the red objective, and the right number is there in the output, but the test still says `expected 2, got nil`.

**Print is for people.** `print` shows a value on the screen, and that is all it does: the value goes to the output and is gone. The code that called the function gets nothing back, which in Lua means `nil`.

**Return is for code.** `return` hands the value back to the caller, which can store it, compare it or do maths with it. Other code can only use what a function returns, never what it printed.

```lua
function M.tip_shown(total)
  print(total / 10)       -- shows 2 on screen, hands back nothing
end

function M.tip(total)
  return total / 10       -- hands 2 back: the caller can add it up
end
```

**A rule of thumb.** A function that works something out should return it, and only the code at the very top should decide what to print. Then every function can be tested, and the printing happens in one place.

**Use what comes back.** A call is an expression, so its answer can go anywhere a value can: `local bill = M.pizza_price(2) + M.tip(20)`. That only works when both functions return.

## Your turn

- Save and look at what `delivery_fee` prints, and what the test got back.
- Make `delivery_fee` hand the fee back instead of printing it.
- In `order_total`, give back `food` plus the delivery fee for `km`.
