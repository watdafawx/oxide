**Why now:** two hash workers finish a jar at the same moment and both add 1 to "files done". Without a lock one update can vanish. A **`Mutex`** makes them take turns.

```rust
use std::sync::Mutex;

let done = Mutex::new(0u32);

{
    let mut guard = done.lock().unwrap(); // wait for our turn
    *guard += 1;                          // change it
}                                         // guard dropped: lock released

println!("{}", done.lock().unwrap());     // 1
```

- `lock()` returns a **guard**. The guard derefs to the value and unlocks when it is dropped.
- To empty a locked `Vec` and keep what was in it, use **`std::mem::take(&mut *guard)`**: it swaps in an empty `Vec` and hands you the old one. (`*guard` reaches the `Vec`, `&mut` borrows it.)
- Keep the guard alive as briefly as you can. Locking again while you still hold it **deadlocks**.
- If a worker panics while holding the lock, the Mutex is **poisoned**: `lock()` returns `Err`. `into_inner()` on that error  still gives you the guard.
- **`unwrap_or_else(|e| ...)`** is `unwrap` with a fallback: on `Err(e)` it runs your closure with the error inside instead of panicking. Example: `"x".parse::<i32>().unwrap_or_else(|_| 0)` is `0`.
- Tests here run your code on a helper thread with a timeout, so a deadlock shows as a failure.

## Your turn

In `src/lib.rs`:

- `mark_hashed`: add 1 to the files-done counter under the lock.
- `log_line` and `take_log`: append a status line; hand over all lines and leave the log empty.
- `hashed_count_even_if_poisoned`: read the value even when a panic poisoned the lock.
