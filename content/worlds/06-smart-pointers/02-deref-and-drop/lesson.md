Smart pointers are structs that act like pointers. Two traits make that work: **`Deref`** (`*x` and auto-coercion) and **`Drop`** (cleanup code).

```rust
use std::ops::Deref;

struct Wrapper(String);

impl Deref for Wrapper {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

struct Noisy;
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("bye");
    }
}

let w = Wrapper(String::from("hi"));
println!("{}", w.len()); // method found through Deref
```

- `Deref` lets `&Wrapper` coerce to `&String`, then to `&str`.
- `drop` runs automatically when a value goes out of scope, in reverse order of creation.
- You never call `.drop()` yourself. To end a value early use **`drop(x)`**, the free function.
- A value holding a `&mut` keeps it borrowed until the value is dropped.

## Your turn

In `src/lib.rs`:

- Implement `Deref for MyBox<T>`.
- Fill in `Drop for Guard`: push `"drop <name>"` onto the log.
- Write `early_release`: make a `Guard`, drop it explicitly, *then* push `"work"`.
