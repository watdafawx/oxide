Threads that share data need a rule: one at a time. A **`Mutex<T>`** is that rule. The data lives *inside* the mutex, so you cannot touch it without locking.

```rust
use std::sync::Mutex;

let name = Mutex::new(String::from("hi"));

{
    let mut guard = name.lock().unwrap();  // blocks until the lock is free
    guard.push('!');                       // guard acts like &mut String
}                                          // guard dropped here: unlocked

println!("{}", name.lock().unwrap());      // hi!
```

- `lock()` returns a **guard**. The lock is held as long as the guard lives, and released when it is dropped. Keep guards short-lived.
- Locking twice on the same thread without dropping the first guard is a **deadlock**: it waits for itself forever.
- **Poisoning**: if a thread panics while holding the lock, the mutex is poisoned and later `lock()` calls return `Err`, because the data may be half-updated. `unwrap()` passes the panic along; `into_inner()` on the error gets the data anyway.
- A `Mutex` on its own only works in one place. To share it between threads you wrap it in `Arc` (next quest).

## Your turn

In `src/lib.rs`:

- `bump`: lock the counter and add 1.
- `push_line` and `take_all`: append to a shared log; take everything out and leave it empty.
- `read_even_if_poisoned`: return the value even if the lock is poisoned.
