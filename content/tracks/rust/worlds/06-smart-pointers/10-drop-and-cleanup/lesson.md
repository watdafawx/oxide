Cleanup should not be something you remember to do. **`Drop`** runs code automatically when a value goes out of scope: closing files, releasing locks, logging.

```rust
struct Noisy;
impl Drop for Noisy {
    fn drop(&mut self) {
        println!("bye");
    }
}

{
    let _n = Noisy;
    println!("working");
}                      // "bye" prints here, at the closing brace
```

- `drop` runs automatically when a value goes out of scope, in reverse order of creation.
- You never call `.drop()` yourself. To end a value early use **`drop(x)`**, the free function.
- A value holding a `&mut` keeps it borrowed until the value is dropped.

## Your turn

In `src/lib.rs`:

- Fill in `Drop for Guard`: push `"drop <name>"` onto the log.
- Write `early_release`: make a `Guard`, drop it explicitly, *then* push `"work"`.
