**Why now:** when you remove an inserter from the factory it should switch itself off in the log, without you remembering to. **`Drop`** runs code when a machine is removed. Drop order is power-down order.

```rust
struct Lamp;
impl Drop for Lamp {
    fn drop(&mut self) {
        println!("power down");
    }
}

{
    let _lamp = Lamp;
    println!("working");
}                      // "power down" prints here, at the closing brace
```

- `drop` runs automatically when a value goes out of scope, in reverse order of creation: the last machine placed powers down first.
- You never call `.drop()` yourself. To end a value early use **`drop(x)`**, the free function.
- A value holding a `&mut` keeps it borrowed until the value is dropped.

## Your turn

In `src/lib.rs`:

- Fill in `Drop for Inserter`: push `"power down <name>"` onto the log.
- Write `hot_swap`: make an `Inserter`, drop it explicitly, *then* push `"place new inserter"`.
