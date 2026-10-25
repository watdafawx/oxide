The full order price has three rules: the pizzas with their toppings, 10% off when the food costs more than 50, and the delivery fee. One long function that does all of that is hard to read and harder to fix. Split it into small ones.

**One job each.** Give every rule its own small function, with a name that says the job: `food_price`, `discount`, `delivery_fee`. Then the main function reads like the rules themselves: food, minus the discount, plus delivery.

```lua
function M.day_cost(hours)
  return M.wages(hours) + M.rent()      -- reads like the rule
end
```

**Test the pieces.** Small functions can be checked one at a time, and here each one has its own objective. When the total is wrong, the pieces' tests tell you which rule broke, instead of leaving you to search one long function.

**Build up.** Write the smallest piece first and get it green, then the next one that uses it. Each step stands on something you already know works. The starter already has `delivery_fee` from the last quest.

**Name the steps.** Splitting a problem like this is called **decomposition**: breaking it into parts small enough to solve one at a time. It is the core skill of programming, in every language. The syntax is the easy part.

## Your turn

- `food_price`: every pizza costs 7 plus 1.5 per topping.
- `discount`: a tenth of `food` when it is over 50, otherwise 0.
- `order_total`: the food price, minus its discount, plus the delivery fee for `km`, using the three functions.
