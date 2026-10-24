Zooming scales every point by a number, and it should read `v * 2.5`. This time the right-hand side is an `f64`, not another vector, and the trait has a place to say so.

**The other side is a parameter.** `Mul` is really `Mul<Rhs>`, where **`Rhs`**, the right-hand side, is a generic parameter. `impl Mul<f64> for Vec2` reads "a `Vec2` times an `f64`". In `Add` you never wrote it because it defaults to the same type.

**Generic or associated?** `Rhs` is a generic because one type can be multiplied by many things, an `f64` or another `Vec2`, each with its own `impl`. `Output` is associated because once both sides are known there is one answer. That is the rule from the associated-type quest, in one trait.

```rust
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coins(u32);

impl Mul<u32> for Coins {           // Coins * u32
    type Output = Coins;
    fn mul(self, k: u32) -> Coins {
        Coins(self.0 * k)
    }
}
// Coins(3) * 4 == Coins(12)
```

**The other order.** `2.5 * v` calls `f64`'s `mul`, so it needs `impl Mul<Vec2> for f64`, a trait from the standard library on a type from the standard library. That is allowed because your `Vec2` is part of it.

**For the curious: the orphan rule.** You may write `impl Trait for Type` only if the trait or the type is yours, or one of yours appears in it, as `Vec2` does in `Mul<Vec2>`. `impl Display for Vec<u32>` is refused: two crates could both write it, and Rust could not pick one. The guarantee of one impl per trait and type, across the whole program, is called **coherence**.

## Your turn

- In `impl Mul<f64> for Vec2`, multiply both parts by `k`.
- In `impl Mul<Vec2> for f64`, do the same: here `self` is the number.
