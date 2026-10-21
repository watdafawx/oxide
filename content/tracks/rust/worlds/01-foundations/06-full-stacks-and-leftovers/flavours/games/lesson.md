You mined 150 cobblestone, and a stack holds 64. Your calculator has to say how many full stacks that makes and how many blocks are left loose. Two small operators answer that.

**Divide whole numbers.** `/` between two whole numbers gives a whole number and drops whatever does not fit. `100 / 64` is `1`, not `1.56`: one full stack.

**What is left.** `%` gives the **remainder**: what is left after the full stacks are taken out. `100 % 64` is `36`.

**They belong together.** One stack of 64 plus 36 loose adds back up to 100. `/` counts how many fit, `%` says what does not:

```rust
let total = 100;
let full = total / 64;    // 1
let loose = total % 64;   // 36
```

**Exact fits.** When everything fits, nothing is left. `128 % 64` is `0`, and `128 / 64` is `2`.

## Your turn

- In `full_stacks`, return how many times 64 fits into `items`.
- In `loose_items`, return what is left over.
- Save. Both objectives turn green.
