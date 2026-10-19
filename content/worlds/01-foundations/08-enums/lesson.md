A value that is *one of a few kinds* is an **enum**. Each variant can carry its own data.

```rust
enum Pet {
    Fish,                       // no data
    Dog(String),                // one value
    Bird { name: String, wings: u8 },   // named fields
}

impl Pet {
    fn legs(&self) -> u32 {
        match self {            // pick the code by variant
            Pet::Fish => 0,
            Pet::Dog(_) => 4,
            Pet::Bird { .. } => 2,
        }
    }
}
```

- Create one with `Pet::Dog("rex".to_string())`.
- `match` is how you open an enum. The next quest covers it fully; here it works like a switch that hands you the data.
- Methods go in `impl Pet`, same as structs. `self` here is a borrowed `&Pet`, so `match self` binds references.
- `_` means "I don't need this value".

## Your turn

- Open `src/lib.rs`. The `Shape` enum is given.
- Fill `area` (circle uses `PI`) and `name`.
