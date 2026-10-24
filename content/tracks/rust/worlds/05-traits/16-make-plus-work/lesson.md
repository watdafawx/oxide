Moving a point by an offset should read `point + offset`, not `point.add_vec(offset)`. In Rust, `+` is not built into the language for your types: it is a trait, and you can implement it.

**+ is a method call.** `a + b` is short for `a.add(b)` from the **`Add`** trait in `std::ops`. Implement `Add` for `Vec2` and `+` works on it. Every operator has a trait: `Sub` for `-`, `Mul` for `*`, and `Neg` for a `-` in front of one value.

**Output says what comes out.** `Add` has an associated type, `type Output`, for the result of `a + b`. For vectors it is `Vec2` again, but it doesn't have to be.

**By value, on purpose.** `fn add(self, other: Vec2)` takes both sides by value, like `swap` did, because `+` builds a new value from two old ones. `Vec2` derives `Copy`, so taking it is a cheap copy and `a` still works after `a + b`.

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coins(u32);

impl Add for Coins {
    type Output = Coins;
    fn add(self, other: Coins) -> Coins {
        Coins(self.0 + other.0)
    }
}
// Coins(3) + Coins(4) == Coins(7)
```

**Only where it reads like maths.** Implement an operator when its meaning is obvious, as it is for vectors. `+` on two layers would make every reader guess.

## Your turn

- In `impl Add for Vec2`, add the `x` parts and the `y` parts.
- In `impl Neg for Vec2`, flip the sign of both parts.
