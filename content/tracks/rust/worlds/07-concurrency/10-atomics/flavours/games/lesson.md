**Why now:** handing out job ids and counting hashed files with a `Mutex` works, but it makes workers queue for a lock just to add 1. An **atomic** does the add in a single CPU step that no other thread can interrupt.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let hashed = AtomicUsize::new(0);
let before = hashed.fetch_add(1, Ordering::Relaxed); // returns the OLD value: 0
println!("{before} {}", hashed.load(Ordering::SeqCst)); // 0 1
```

- `fetch_add` adds and returns the **previous** value. `load` and `store` read and write.
- **`compare_exchange(current, new, ..)`**: set to `new` only if the value is still `current`. Exactly one racing thread succeeds. That is how you "claim" something once.
- Atomics are `Sync`, so share them through `&` or `Arc`: no lock and no `mut`.
- `Ordering`: when unsure use `SeqCst` (strongest). `Relaxed` is fine for plain counters that nothing else depends on.
- Atomics suit single numbers and flags. For several values that must change together, use a `Mutex`.

## Your turn

In `src/lib.rs`:

- `next_job_id`: `fetch_add(1)` and return the id from before.
- `count_hashed`: workers share one `AtomicUsize` and bump it; return the final value.
- `report_first_failure`: flip a flag from false to true; only the worker that flipped it returns `true`.
