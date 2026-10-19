`Vec<T>` and `Option<T>` hold any type. You can build the same thing: put a type parameter on a **struct**.

```rust
use std::fmt::Display;

struct Wrapper<T> {
    value: T,
}

impl<T> Wrapper<T> {                       // methods for every T
    fn new(value: T) -> Self { Wrapper { value } }
}

impl<T: Display> Wrapper<T> {              // only when T can be printed
    fn show(&self) -> String { format!("[{}]", self.value) }
}
```

- `impl<T>` declares `T`, then `Wrapper<T>` uses it.
- A **bounded impl block** (`impl<T: Display>`) gives methods only to types that qualify.
- `Wrapper<i32>` has `show`. `Wrapper<Vec<i32>>` does not: no `Display`.

## Your turn

- Open `src/lib.rs`.
- Write `new` and `swap` (any `T`), `larger` (needs `PartialOrd`) and `describe_larger` (needs `Display` too).
