A converter turns one thing into another: degrees into degrees, text into a word count. Each converter's answer has its own type, and the trait has to let each one say which.

**Try a fixed type.** `fn convert(&self) -> f64` suits temperatures, but a word count is a `usize`, not a decimal. The trait can't pick one return type that suits everyone.

**Let each impl choose.** An **associated type** is a type slot inside the trait: `type Output;`. Each `impl` fills it in, `type Output = f64;`, and the methods name it as `Self::Output`.

**Why not Convert<Output>?** A generic trait could carry the type too, but then `Celsius` could implement `Convert<f64>` *and* `Convert<String>`, and every call would have to say which one it meant. An associated type means one answer per type, so `c.convert()` needs no extra words.

```rust
use std::fmt::Display;

trait Measure {
    type Unit;                         // each impl picks one
    fn measure(&self) -> Self::Unit;
}

struct Rope(u32);

impl Measure for Rope {
    type Unit = u32;
    fn measure(&self) -> u32 { self.0 * 100 }
}

fn show<M>(m: &M) -> String
where
    M: Measure,
    M::Unit: Display,                  // a bound on the slot
{
    format!("{} cm", m.measure())
}
```

**In generic code.** In `fn report<C: Convert>`, `C::Output` means "whatever `C` chose". To print it, put a bound on it: `where C::Output: Display`.

**You will see it everywhere.** `Iterator` has `type Item` and `Add` has `type Output`. The next quests use both.

## Your turn

- In `impl Convert for Celsius`, write `convert`: Fahrenheit is `c * 9 / 5 + 32`.
- In `impl Convert for Words`, write `convert`: the number of words.
- Write `report`: `result: <output>`.
