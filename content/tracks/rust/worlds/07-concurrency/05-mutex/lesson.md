**Why now:** the hasher's workers all report into one shared log; a `Mutex` makes sure they take turns writing it.

Threads that share data need a rule: one at a time. A **`Mutex<T>`** is that rule. The data lives *inside* the mutex, so you cannot touch it without locking.

```rust
use std::sync::Mutex;

let log = Mutex::new(String::from("hashed: "));

{
    let mut guard = log.lock().unwrap();   // blocks until the lock is free
    guard.push_str("sodium.jar");          // guard acts like &mut String
}                                          // guard dropped here: unlocked

println!("{}", log.lock().unwrap());       // hashed: sodium.jar

// a number works the same, with * to reach through the guard:
let count = Mutex::new(0u32);
*count.lock().unwrap() += 1;               // lock, change, guard dropped at the `;`
```

- `lock()` returns a **guard**. The lock is held as long as the guard lives, and released when it is dropped. Keep guards short-lived.
- To empty a locked `Vec` and keep what was in it, use **`std::mem::take(&mut *guard)`**: it swaps in an empty `Vec` and hands you the old one. (`*guard` reaches the `Vec`, `&mut` borrows it.)
- Locking twice on the same thread without dropping the first guard is a **deadlock**: it waits for itself forever.
- **Poisoning**: if a thread panics while holding the lock, the mutex is poisoned and later `lock()` calls return `Err`, because the data may be half-updated. `unwrap()` passes the panic along; `into_inner()` on the error gets the data anyway.
- **`unwrap_or_else(|e| ...)`** is `unwrap` with a fallback: on `Err(e)` it runs your closure with the error inside instead of panicking. Example: `"x".parse::<i32>().unwrap_or_else(|_| 0)` is `0`.
- A `Mutex` on its own only works in one place. To share it between threads you wrap it in `Arc` (next quest).

## Your turn

In `src/lib.rs`:

- `bump`: lock the counter and add 1.
- `push_line` and `take_all`: append to a shared log; take everything out and leave it empty.
- `read_even_if_poisoned`: return the value even if the lock is poisoned.
