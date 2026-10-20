`a + b` is not magic: it calls `a.add(b)` from the **`Add` trait**. Implement the trait and your type gets the operator.

```rust
use std::ops::{Add, Neg};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct P { x: i32 }

impl Add for P {
    type Output = P;                          // what `a + b` produces
    fn add(self, other: P) -> P { P { x: self.x + other.x } }
}

impl Neg for P {
    type Output = P;
    fn neg(self) -> P { P { x: -self.x } }
}

impl PartialOrd for P {                       // powers <, >, <=, >=
    fn partial_cmp(&self, other: &P) -> Option<Ordering> {
        self.x.partial_cmp(&other.x)
    }
}
// P { x: 1 } + P { x: 2 } == P { x: 3 },  -P { x: 4 } == P { x: -4 }
```

- Each operator has a trait in `std::ops`: `Add`, `Sub`, `Mul`, `Neg`...
- `Mul<f64>` means "multiply by an `f64`": the right-hand type is the generic.
- `PartialOrd` needs `PartialEq` too (derive it).

- **Careful:** `<=` here compares by length, but the derived `==` compares every field. Two different vectors of equal length are `<=` each other without being `==`. Real code keeps the two consistent; this quest orders by length on purpose.

## Your turn

- Open `src/lib.rs`.
- Fill in `Add`, `Neg`, `Mul<f64>` and `PartialOrd` for `Vec2`. Vectors are ordered by **length**.
