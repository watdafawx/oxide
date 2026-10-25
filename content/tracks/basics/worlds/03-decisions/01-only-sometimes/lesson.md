The roller coaster's gate shows a message as each rider steps up, and riders under 120 cm must be stopped. Every program so far ran every line, every time. This one needs a line that runs only sometimes.

**Run it only if.** `if height < 120 then ... end` runs the lines between `then` and `end` only when the comparison is `true`. When it is `false`, Lua jumps past the `end` and carries on with the next line.

**Return leaves early.** `return` ends the function on the spot. So in the code below, when `bags > 2` the function hands back "Too many bags" and the last line never runs. When it is false, the `if` is skipped and the last line runs. The shape is: the special case first, the usual case last.

```lua
function M.bag_check(bags)
  if bags > 2 then
    return "Too many bags"
  end
  return "Go ahead"
end
```

**Indent the inside.** The lines inside the `if` are pushed in by two spaces. Lua does not need that, but people do: it shows at a glance which lines depend on the condition. Rust and Java mark the inside with curly brackets `{ }` instead of `then` and `end`, and indent it the same way.

**The condition.** The part between `if` and `then` is the **condition**. It can be any expression that works out to true or false: a comparison, or a name that holds a boolean.

**Not only returns.** The inside of an `if` can hold any lines, not just a `return`. `if age < 12 then price = 5 end` changes a variable only for children, and the function carries on either way.

## Your turn

- In `gate_message`, return `Too short, sorry` when `height` is under 120.
- Otherwise it returns `Enjoy the ride`.
- In `fast_pass_price`, lower the price to 5 for children under 12.
