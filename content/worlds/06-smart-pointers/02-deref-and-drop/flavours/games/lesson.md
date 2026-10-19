**Why now:** you will pass machines around behind handles, and you want a removed inserter to switch itself off in the log. **`Deref`** makes a handle act like what it wraps; **`Drop`** runs code when a machine is removed. Drop order is power-down order.

```rust
use std::ops::Deref;

struct Handle(String);

impl Deref for Handle {
    type Target = String;
    fn deref(&self) -> &String {
        &self.0
    }
}

struct Lamp;
impl Drop for Lamp {
    fn drop(&mut self) {
        println!("power down");
    }
}

let h = Handle(String::from("fast-inserter"));
println!("{}", h.len()); // method found through Deref
```

- `Deref` lets `&Handle` coerce to `&String`, then to `&str`.
- `drop` runs automatically when a value goes out of scope, in reverse order of creation: the last machine placed powers down first.
- You never call `.drop()` yourself. To end a value early use **`drop(x)`**, the free function.
- A value holding a `&mut` keeps it borrowed until the value is dropped.

## Your turn

In `src/lib.rs`:

- Implement `Deref for InserterHandle<T>`.
- Fill in `Drop for Inserter`: push `"power down <name>"` onto the log.
- Write `hot_swap`: make an `Inserter`, drop it explicitly, *then* push `"place new inserter"`.
