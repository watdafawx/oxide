One file in the folder is broken and its worker panics in the middle of updating the counter, while it holds the lock. Every other worker is fine, and the summary screen still wants to show how many files were done. The `unwrap()` on `lock()` now has a reason to exist.

**The lock is poisoned.** If a thread panics while it holds a guard, the guard is still dropped and the lock opens, but the mutex remembers. It is now **poisoned**, and every later `lock()` returns `Err(PoisonError)` instead of `Ok(guard)`. The reason: the panic may have struck halfway through a change, so the value inside might be half-updated.

**unwrap passes the panic on.** With `lock().unwrap()`, every thread that touches a poisoned mutex panics too. That is a safe default: one broken update stops everyone from trusting bad data. But a counter is a single number, and it cannot be half-written, so here you may decide to read it anyway.

**Get the guard anyway.** The `PoisonError` still holds the guard, and `e.into_inner()` takes it out. To use it, swap `unwrap()` for `unwrap_or_else(closure)`: on `Ok(v)` it gives `v`, and on `Err(e)` it calls your closure with `e` and gives whatever the closure returns. It works on any `Result`.

```rust
let count: u32 = "12x".parse().unwrap_or_else(|_e| 0);   // not a number: 0
let mut log = log_lock.lock().unwrap_or_else(|e| e.into_inner());
```

## Your turn

- In `files_done`, lock the counter so that a poisoned lock still gives you the guard, and return the number inside.
