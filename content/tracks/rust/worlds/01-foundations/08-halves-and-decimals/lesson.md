Printing needs 1.5 ml of ink per sheet, and you want half a pallet. Whole numbers cannot say 1.5, so the planner needs a second type of number.

**Numbers with a point.** `f64` is a number with a decimal point: `1.5`, `0.25`, `64.0`. Use it whenever a fraction makes sense. It is written with a point even when it is a whole value, so one is `1.0`.

**Try the obvious thing.** Say `sheets` is an `f64` and you write `sheets / 2`. Rust refuses, because the 2 is a whole number and the sheets are not:

```text
error[E0277]: cannot divide `f64` by `{integer}`
help: consider using a floating-point literal by writing it with `.0`
```

**Write the point.** The fix is the one the message offers: `2.0`. In `f64` maths every number needs its point, even the whole ones. Then both sides are the same type and the maths works.

```rust
let cost = sheets * 2.5;
let half = sheets / 2.0;
```

**Keep to one type.** An `f64` and a `u32` do not mix in one sum. For now, keep a calculation in one type from start to finish. A later quest shows how to convert between them.

## Your turn

- In `ink_ml`, return the sheets times 1.5.
- In `half_pallet`, return half of `sheets`.
- Save. Both objectives turn green.
