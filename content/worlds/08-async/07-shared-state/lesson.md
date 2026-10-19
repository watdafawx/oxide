**Why now:** the downloader's "files done" counter and its cache of mod metadata are read and written by many tasks at once, so they sit behind an `Arc<Mutex>`.

Wrap shared data: **`Arc<Mutex<T>>`**. `Arc` shares ownership across tasks, `Mutex` lets one task at a time change it.

Two Mutexes exist:

- **`std::sync::Mutex`**: fast, but its guard cannot be held across an `.await`. Use it for tiny, quick edits: lock, change, unlock, no await in between.
- **`tokio::sync::Mutex`**: `.lock().await`, and the guard **can** live across `.await`. Use it when you must await while holding the lock.
- **`RwLock`**: many readers **or** one writer. Good for caches that are read much more than written.

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

let done = Arc::new(Mutex::new(0u32));
let t = Arc::clone(&done);
let h = tokio::spawn(async move {
    let mut guard = t.lock().await; // wait for our turn
    *guard += 1;
}); // guard dropped here: lock released
h.await.unwrap();
assert_eq!(*done.lock().await, 1);
```

Keep locks short. Holding one across a slow call makes every other task queue behind it.

## Your turn

In `src/lib.rs`:

- `bump`: lock the counter, read it, `yield_now().await`, write it back plus one. (The read-then-write on purpose: it only works if the lock is held the whole time.)
- `count_concurrently`: run `tasks` tasks, each calling `bump` `per_task` times. Return the total.
- `Cache`: finish `insert` (write lock), `get` and `count` (read lock).
