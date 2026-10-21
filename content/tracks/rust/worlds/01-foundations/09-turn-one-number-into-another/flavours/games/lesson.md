The calculator counts whole items, but an average of two stack counts is not whole: 3 and 4 average 3.5. To get it you have to carry a whole number into the world of decimals.

**Averages are not whole.** `(3 + 4) / 2` with whole numbers gives 3 and loses the half. The average has to be an `f64`, and the sum has to become one first.

**Try the obvious thing.** With `a` and `b` as `u32`, write `(a + b) / 2.0`. Rust refuses to mix the types:

```text
error[E0277]: cannot divide `u32` by `{float}`
```

**The word for it.** `as` converts one type of number into another: `total as f64` is the same value as an `f64`. It is a **cast**.

**Convert, then divide.** Put the cast on the sum, and the division then happens between two `f64` numbers:

```rust
let total = a + b;
let average = total as f64 / 2.0;
```

**Going back cuts.** A cast the other way chops the decimals off. It does not round: `7.9 as u32` is `7`. Use it when you only want the whole part.

## Your turn

- In `average_stack`, add the two counts, convert the sum with `as f64`, and divide by 2.0.
- In `whole_planks`, cast the `f64` to `u32`.
- Save. Both objectives turn green.
