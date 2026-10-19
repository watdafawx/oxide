Some traits need to name a type or a constant of their own: "what does this convert **to**?", "what is this thing called?". Put them inside the trait.

```rust
trait Shrink {
    const FACTOR: u32;                     // associated const: one value per type
    type Output;                           // associated type: one type per impl
    fn shrink(&self) -> Self::Output;
}

struct Big(u32);

impl Shrink for Big {
    const FACTOR: u32 = 10;
    type Output = u32;
    fn shrink(&self) -> u32 { self.0 / Self::FACTOR }
}

fn run<S: Shrink>(s: &S) -> S::Output { s.shrink() }   // S::Output is Big's u32
// run(&Big(50)) == 5,  Big::FACTOR == 10
```

- **`type Output`**: each implementor picks its own return type.
- **`const NAME`**: a value attached to the type, read as `Type::NAME` or `T::NAME`.
- In generic code, `T::Output` means "whatever `T` chose".

## Your turn

- Open `src/lib.rs`.
- For `Celsius` and `Words`: set `NAME` and write `convert`.
- Write `label` so it works for any `Convert` whose output can be displayed.
