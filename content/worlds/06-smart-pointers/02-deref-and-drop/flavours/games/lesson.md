**Why now:** you will pass machines around behind handles. **`Deref`** makes a handle act like what it wraps, so `*handle` works and `&InserterHandle<String>` can go where a `&str` is expected.

```rust
use std::ops::Deref;

struct Handle(String);

impl Deref for Handle {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

let h = Handle(String::from("fast-inserter"));
println!("{}", h.len()); // method found through Deref
```

- `Deref` lets `&Handle` coerce to `&String`, then to `&str`.
- `*h` is sugar for `*h.deref()`.
- `type Target` says what the handle points at.

## Your turn

- In `src/lib.rs`, implement `Deref for InserterHandle<T>`.
