A delivery brings 150 sheets, and a pallet holds 64. Your planner has to say how many full pallets that fills and how many sheets are left loose. Two small operators answer that.

**Divide whole numbers.** `/` between two whole numbers gives a whole number and drops whatever does not fit. `100 / 64` is `1`, not `1.56`: one full pallet.

**What is left.** `%` gives the **remainder**: what is left after the full pallets are taken out. `100 % 64` is `36`.

**They belong together.** One pallet of 64 plus 36 loose adds back up to 100. `/` counts how many fit, `%` says what does not:

```rust
let total = 100;
let full = total / 64;    // 1
let loose = total % 64;   // 36
```

**Exact fits.** When everything fits, nothing is left. `128 % 64` is `0`, and `128 / 64` is `2`.

## Your turn

- In `full_pallets`, return how many times 64 fits into `sheets`.
- In `loose_sheets`, return what is left over.
- Save. Both objectives turn green.
