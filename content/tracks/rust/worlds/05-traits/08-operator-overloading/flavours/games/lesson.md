**Why now:** every mob has a position and a velocity, and `pos = pos + velocity * dt` should read like that, not `pos.add(velocity.scale(dt))`. `a + b` is not magic: it calls `a.add(b)` from the **`Add` trait**. Implement the trait and `Vec3` gets the operator.

```rust
use std::ops::{Add, Mul};
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Step { dx: i32 }

impl Add for Step {
    type Output = Step;                       // what `a + b` produces
    fn add(self, other: Step) -> Step { Step { dx: self.dx + other.dx } }
}

impl Mul<i32> for Step {                      // right-hand side is an i32, not a Step
    type Output = Step;
    fn mul(self, k: i32) -> Step { Step { dx: self.dx * k } }
}

impl PartialOrd for Step {                    // powers <, >, <=, >=
    fn partial_cmp(&self, other: &Step) -> Option<Ordering> {
        self.dx.partial_cmp(&other.dx)
    }
}
// Step { dx: 1 } + Step { dx: 2 } == Step { dx: 3 },  Step { dx: 2 } * 3 == Step { dx: 6 }
```

- Each operator has a trait in `std::ops`: `Add`, `Sub`, `Mul`, `Neg`...
- `Mul<f64>` means "multiply by an `f64`": the right-hand type is the generic. `Neg` is the same shape with one operand: `fn neg(self) -> Self::Output`.
- `PartialOrd` needs `PartialEq` too (derive it).

## Your turn

- Open `src/lib.rs`.
- Fill in `Add`, `Neg`, `Mul<f64>` and `PartialOrd` for `Vec3`. Vectors are ordered by **length**, so "closer" means smaller.
