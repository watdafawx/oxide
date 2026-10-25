The gate's light is green or red, never both and never neither. When there are exactly two outcomes, writing both of them down makes the code much harder to get wrong.

**Otherwise.** `else` adds the other branch: `if open then ... else ... end`. Exactly one of the two parts runs, every time. Each part is called a **branch**, like a path that splits in two.

```lua
if temperature > 30 then
  drink = "water"
else
  drink = "tea"
end
```

**One end.** The whole `if ... else ... end` is one piece with one `end`. `else` has no condition of its own; it simply means "every case the condition did not catch".

**Why not two ifs.** Two separate checks, `if x > 30` and `if x < 30`, can both miss: at exactly 30 neither runs, and the drink is never chosen. `else` cannot miss, because it covers whatever the first branch left over.

**Trace the edge.** To check an `if`, pick a value, decide whether the condition is true, and follow only that branch. Then pick a value right at the edge, like exactly 120. Edges are where bugs live, which is why the tests always try them.

## Your turn

- In `light_colour`, return `green` from 120 cm up, and `red` otherwise.
- In `photo_price`, return 8 when `wants_print` is true, and 3 otherwise.
- Save. The tests try exactly 120 too.
