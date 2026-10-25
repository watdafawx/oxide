A pizza's price depends on how many toppings it has. World 2 called the value in the brackets an input. Programmers have two more precise words for it, and documentation and error messages use both.

**Parameter.** In the definition, a name in the brackets is a **parameter**: `function M.pizza_price(toppings)`. It is a variable that gets its value from the call. The definition does not know the number yet; it only names the slot.

**Argument.** In the call, the value you put in the brackets is an **argument**: in `M.pizza_price(3)`, the argument is 3, and for that call the parameter `toppings` holds 3. Parameter: the slot. Argument: what goes into it.

```lua
function M.drink_price(size, cans)   -- two parameters
  if size == "large" then
    return cans * 3
  end
  return cans * 2
end

local cost = M.drink_price("large", 4)   -- two arguments: 12
```

**In order.** Arguments fill parameters by position: the first argument goes into the first parameter, the second into the second. Names in the call do not matter, only the order, so swap two arguments and the function gets them swapped.

**Missing ones.** Pass fewer arguments than there are parameters and the extra parameters are `nil`. Rust and Java refuse a call like that before the program runs. In Lua, the usual sign is a nil turning up in a sum: `attempt to perform arithmetic on local 'cans' (a nil value)`.

## Your turn

- Define `M.pizza_price` with one parameter, `toppings`: 7 for the base plus 1.5 per topping.
- Define `M.order_price` with two parameters, `pizzas` and `toppings`, in that order: every pizza costs the same.
- Save. The tests call both with several arguments.
