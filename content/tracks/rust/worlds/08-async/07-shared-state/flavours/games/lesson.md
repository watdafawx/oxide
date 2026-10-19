**Why now:** every download task bumps one "steps done" counter, and all of them look up and record which version of a mod is already installed. Tasks share state, and `std::sync::Mutex` is the wrong tool when you must hold a lock **across an `.await`**.

```rust
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

let steps = Arc::new(Mutex::new(0usize));
{
    let mut guard = steps.lock().await;   // async lock: waits without blocking the thread
    *guard += 1;
}                                          // released here

let versions = Arc::new(RwLock::new(std::collections::HashMap::new()));
versions.write().await.insert("jei", "1.21");   // one writer
let v = versions.read().await.get("jei").copied(); // many readers
```

- `tokio::sync::Mutex::lock()` is `async`. A task that waits for it lets others run.
- Holding a `std::sync::MutexGuard` across `.await` can deadlock the runtime. A tokio guard is safe to hold, but keep it short anyway.
- A lost update comes from **read, wait, write** without the lock held the whole time. The test makes tasks yield in the middle to expose it.
- `RwLock` suits a cache: many readers, a few writers.
- Share through `Arc`, and `Arc::try_unwrap(..)` once all tasks are joined.

## Your turn

In `src/lib.rs`:

- `record_step`: lock, read the value, `yield_now().await`, write `value + 1`.
- `finish_downloads`: spawn `tasks` tasks that each call `record_step` `steps_each` times; return the final count.
- `VersionCache`: `insert`, `get`, `count` over the `RwLock`.
