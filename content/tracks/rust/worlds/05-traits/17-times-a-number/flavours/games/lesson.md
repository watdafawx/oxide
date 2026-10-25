A tick moves a mob by `velocity * dt`, where `dt` is the length of a tick in seconds. This time the right-hand side is an `f64`, not another vector, and the trait has a place to say so.

**The other side is a parameter.** `Mul` is really `Mul<Rhs>`, where **`Rhs`**, the right-hand side, is a generic parameter. `impl Mul<f64> for Vec3` reads "a `Vec3` times an `f64`". In `Add` you never wrote it because it defaults to the same type.

**Generic or associated?** `Rhs` is a generic because one type can be multiplied by many things, an `f64` or another `Vec3`, each with its own `impl`. `Output` is associated because once both sides are known there is one answer. That is the rule from the loot quest, in one trait.

```rust
use std::ops::Mul;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Xp(u32);

impl Mul<u32> for Xp {              // Xp * u32
    type Output = Xp;
    fn mul(self, k: u32) -> Xp {
        Xp(self.0 * k)
    }
}
// Xp(3) * 4 == Xp(12)
```

**The other order.** `dt * velocity` calls `f64`'s `mul`, so it needs `impl Mul<Vec3> for f64`, a trait from the standard library on a type from the standard library. That is allowed because your `Vec3` is part of it.

**For the curious: the orphan rule.** You may write `impl Trait for Type` only if the trait or the type is yours, or one of yours appears in it, as `Vec3` does in `Mul<Vec3>`. `impl Display for Vec<u32>` is refused: two mods could both write it, and Rust could not pick one. The guarantee of one impl per trait and type, across the whole program, is called **coherence**.

## Your turn

- In `impl Mul<f64> for Vec3`, multiply all three parts by `k`.
- In `impl Mul<Vec3> for f64`, do the same: here `self` is the number.
