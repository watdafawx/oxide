The converter menu shows a name for each converter. That name belongs to the type, not to one value: every `Celsius` is called the same, and the menu should not need a temperature in hand to ask.

**A value with no self.** An **associated const** is a constant inside the trait: `const NAME: &str;`. Each `impl` sets its own, `const NAME: &str = "word-count";`, the same way it filled in `type Output`.

**Read it from the type.** You write `Words::NAME`, or in generic code `C::NAME`. No `self` is involved, so you don't need a value to ask.

```rust
trait Unit {
    const SYMBOL: &str;
}

struct Metre;

impl Unit for Metre {
    const SYMBOL: &str = "m";
}

fn with_symbol<U: Unit>(n: u32) -> String {
    format!("{n} {}", U::SYMBOL)
}
// with_symbol::<Metre>(5) is "5 m"
```

**Why not a method?** `fn name(&self) -> String` would work too, but it needs a value and builds a new `String` on every call. A const is fixed when the program is built and costs nothing to read.

## Your turn

- In both `impl`s, set `NAME`: `celsius->fahrenheit` and `word-count`.
- Write `label`: `<NAME>: <output>`.
