Back to the "files done" counter that lost updates. Workers must take turns: while one does read, add, write, the others wait. A lock is how you make them, and Rust's lock is built so that you cannot forget to use it.

**The value lives inside.** `Mutex<T>`, short for **mutual exclusion**, wraps a value: `Mutex::new(0)`. There is no way to reach the number except through the mutex, so no thread can touch it without asking first. In other languages the lock and the data are separate, and forgetting the lock compiles fine.

**Lock, get a guard.** `m.lock()` waits until no other thread holds the lock, then hands back a **guard**: a value that acts like a `&mut` to the data inside. `lock()` returns a `Result`; `unwrap()` it for now, and the next quest explains when it is an `Err`. With a number, reach through the guard with `*`, as with any `&mut`: `*guard += 1`.

**Drop unlocks.** The lock is held exactly as long as the guard lives. When the guard is dropped, at the closing brace of its block or at the `;` if you never gave it a name, the lock opens again and the next waiting thread gets in. Keep guards short-lived: hold the lock for the update, not for the slow hashing before it.

**A thread waiting for itself.** Call `lock()` a second time on the same thread while your first guard is still alive, and it waits for the lock to open. It never will, because you are the one holding it. That is a **deadlock**, the race condition from quest 08 that Rust cannot catch for you.

```rust
use std::sync::Mutex;

let total_bytes = Mutex::new(0_u64);
{
    let mut guard = total_bytes.lock().unwrap();  // waits until free
    *guard += 4_096;                              // guard acts like &mut u64
}                                                 // guard dropped: unlocked
println!("{}", total_bytes.lock().unwrap());    // 4096; unlocked again at the ;
```

## Your turn

- In `bump`, lock the counter and add 1 to it through the guard.
- In `log_line`, lock the log and push the line as a `String`.
