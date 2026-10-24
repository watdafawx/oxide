Every mob has a position and a velocity, and one tick should read `pos = pos + velocity`, not `pos.add_vec(velocity)`. In Rust, `+` is not built into the language for your types: it is a trait, and you can implement it.

**+ is a method call.** `a + b` is short for `a.add(b)` from the **`Add`** trait in `std::ops`. Implement `Add` for `Vec3` and `+` works on it. Every operator has a trait: `Sub` for `-`, `Mul` for `*`, and `Neg` for a `-` in front of one value.

**Output says what comes out.** `Add` has an associated type, `type Output`, for the result of `a + b`. For vectors it is `Vec3` again, but it doesn't have to be.

**By value, on purpose.** `fn add(self, other: Vec3)` takes both sides by value, like `swap_hands` did, because `+` builds a new value from two old ones. `Vec3` derives `Copy`, so taking it is a cheap copy and `pos` still works after `pos + velocity`.

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Xp(u32);

impl Add for Xp {
    type Output = Xp;
    fn add(self, other: Xp) -> Xp {
        Xp(self.0 + other.0)
    }
}
// Xp(3) + Xp(4) == Xp(7)
```

**Only where it reads like maths.** Implement an operator when its meaning is obvious, as it is for vectors. `+` on two zombies would make every reader guess.

## Your turn

- In `impl Add for Vec3`, add the `x`, `y` and `z` parts.
- In `impl Neg for Vec3`, flip the sign of all three parts.
