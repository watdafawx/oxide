`largest_i32` and `largest_f64` would be the same code twice. A **generic** function takes a type parameter `T` and works for every type that meets its **bounds**.

```rust
use std::fmt::Display;

fn show<T: Display>(x: T) -> String {         // T must implement Display
    format!("<{}>", x)
}

fn pair_up<A, B>(a: A, b: B) -> String        // `where` moves long bounds out of the way
where
    A: Display,
    B: Display,
{
    format!("{a} & {b}")
}
```

- `<T: Bound>` says "any type `T`, as long as it implements `Bound`".
- Combine bounds with `+`: `T: Display + PartialOrd`.
- Without the bound the compiler refuses `x > y` or `{}`: it cannot know `T` supports it.
- Prefer a **`where`** clause when the bounds get long.

## Your turn

- Open `src/lib.rs`.
- Fill in `largest`, `show_all` and `count_matching`. The signatures and bounds are given.
