The machine cannot have one function for 3 tickets, another for 4 and another for 5. It needs one function that works out the price for any number of tickets, with the number handed in each time it is used.

**A slot for the input.** A name inside the brackets of the `function` line is an **input**: `function M.ticket_cost(tickets)` means "this function needs a value, and calls it `tickets`". Inside the function, `tickets` works like any other variable.

**The caller fills it.** Whoever runs the function puts a value in the brackets: `ticket_cost(3)` runs it with `tickets` set to 3, and `ticket_cost(5)` runs the very same lines with 5. The tests call yours several times, with different numbers.

```lua
function M.popcorn_cost(boxes)
  return boxes * 4        -- popcorn_cost(2) gives 8, popcorn_cost(5) gives 20
end
```

**Why inputs.** Without an input, a function answers one question forever. With one, the same few lines answer every version of the question. That is the whole point of writing the steps down once.

**Two inputs.** A comma separates several inputs: `function M.total(tickets, boxes)` takes two, and they are matched in order, so `total(3, 1)` means 3 tickets and 1 box. World 5 comes back to inputs and gives them their proper names.

## Your turn

- In `ticket_cost`, return the price of `tickets` tickets at 9 each.
- In `order_cost`, add 4 for each popcorn box in `boxes`.
- Save. The tests try several numbers, so a fixed answer will not pass.
