Smart pointers are structs that act like pointers. **`Deref`** is the trait that makes `*x` work on them, and lets `&MyBox<String>` be used where a `&str` is expected (auto-coercion).

```rust
use std::ops::Deref;

struct Wrapper(String);

impl Deref for Wrapper {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

let w = Wrapper(String::from("hi"));
println!("{}", w.len()); // method found through Deref
```

- `Deref` lets `&Wrapper` coerce to `&String`, then to `&str`.
- `*w` is sugar for `*w.deref()`.
- `type Target` says what the pointer points at.

## Your turn

- In `src/lib.rs`, implement `Deref for MyBox<T>`.
