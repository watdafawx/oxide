Every function so far came with its first and last line already written. The pizza shop's calculator needs new ones, so from now on you write whole functions yourself, and see what each part is for.

**Two moments.** A function has two moments. **Defining** it writes the steps down under a name, and nothing runs yet. **Calling** it, by writing its name followed by brackets, runs those steps. A recipe printed in a book is defined; cooking it is the call.

```lua
function M.slices_per_pizza()       -- define: the name, then (inputs)
  return 8                          -- the steps
end                                 -- the end of the definition

local slices = M.slices_per_pizza() -- call: the steps run, 8 comes back
```

**The parts.** `function` starts the definition, then comes the name, then brackets for the inputs (empty here), then the steps, then `end`. The `M.` in front of the name puts the function into the file's package, where the test can find it.

**Calls need brackets.** `M.slices_per_pizza()` with brackets runs the function and gives its answer. Without brackets, `M.slices_per_pizza` is the function itself, not its answer. Comparing that with 8 is always false, which makes it a common slip.

**Why functions.** A name for a piece of work lets you write it once and call it from anywhere, as often as you like. When it needs a fix, you fix one place. It also lets you think about *what* a step does without keeping *how* it does it in your head.

## Your turn

- Save first and read the message: `expected a function, got nil` means there is no function called `base_price` yet.
- Below the comments, define `M.base_price()` so it gives back 7, the price of a plain pizza.
- Define `M.topping_price()` so it gives back 1.5.
