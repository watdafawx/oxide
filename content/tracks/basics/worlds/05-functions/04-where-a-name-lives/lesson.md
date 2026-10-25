The calculator now has a function for the base price and one that adds toppings to it. The second one fails, even though `price` is right there, a few lines up. Where a name can be used is part of the language, and it is there on purpose.

**Try it.** Save the starter and read the message: `attempt to perform arithmetic on global 'price' (a nil value)`. `with_toppings` uses `price`, and `price` clearly exists: it is made inside `base_price`.

**Scope.** A `local` name exists only inside the block where it was made, from its line down to that block's `end`. That stretch of code is the name's **scope**. `price` was made inside `base_price`, so outside that function there is no `price` at all.

```lua
function M.drinks()
  local cans = 4
  return cans          -- cans exists here
end
-- out here, cans does not exist: using it gives nil
```

**Why the walls help.** Scope is a feature, not a nuisance. Every function can use short names like `price` or `total` without caring whether another function uses the same name. Ten functions can have ten different `total`s, and they never get mixed up.

**Global.** A name used without ever being made `local` is a **global**: one box shared by the whole program. That is why the message says "global 'price'": Lua looked there last and found nothing. Globals are risky because any function can change them at any time, so a bug in one place shows up in another.

**Share by handing over.** To get a value from one function into another, return it from the first and call it in the second (or pass it in as an argument). That is the honest way for functions to share: anyone reading the code can see the connection.

## Your turn

- Save and read the error.
- In `with_toppings`, get the base price by calling `M.base_price()` instead of using `price`.
- In `slices_for`, make `total` a local, so nothing leaks out of the function.
