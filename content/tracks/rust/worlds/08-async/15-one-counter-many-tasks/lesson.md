Every download task adds one to the "episodes done" counter. In between reading the counter and writing it back, a task may wait, for example to tell the server it is done. Many tasks, one number, and a wait in the middle: this needs a lock that works with `.await`.

**Share it with Arc.** As with threads in world 07, many tasks can own one value through an `Arc`, and each task gets its own `Arc::clone`. A task must own what it uses (quest 07), so a borrowed `&` to a local counter won't do.

**tokio's Mutex.** `tokio::sync::Mutex` works like the one from world 07, with one change: `lock()` is async. `total.lock().await` waits for its turn by giving the thread back, then hands you a guard. Read and write through `*guard`; the lock opens again when the guard is dropped.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let total = Arc::new(Mutex::new(0u64));
let for_task = Arc::clone(&total);
tokio::spawn(async move {
    let mut guard = for_task.lock().await; // our turn
    *guard += 48_000_000;                  // bytes of one episode
});                                        // guard dropped: unlocked
```

**Why not std's Mutex?** Holding `std::sync::Mutex`'s guard across an `.await` fails twice. Its guard is not `Send`, since an OS lock must be opened by the thread that closed it, while a task may resume on another thread; `tokio::spawn` then refuses with `future cannot be sent between threads safely`. And a task waiting for a `std` lock blocks its whole worker thread (quest 06).

**When std is fine.** For lock, change, unlock with no `.await` in between, `std::sync::Mutex` is faster and is what tokio's own docs suggest. Reach for tokio's lock only when the guard must live across an `.await`.

**The lost update.** Read, wait, write without holding the lock, and another task reads the same old value meanwhile: both write old + 1 and one update is gone. Keeping the guard for the whole read, wait, write prevents it. In the starter, `yield_now().await` stands for the wait: it pauses the task so the others get to run.

## Your turn

- In `record_done`, keep the guard while you read, `yield_now().await`, and write the value plus one.
- In `finish_all`, share one counter with `tasks` tasks that each call `record_done` `per_task` times, await them all, and return the count.
