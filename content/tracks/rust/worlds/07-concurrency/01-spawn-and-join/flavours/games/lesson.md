**Why now:** your mod folder hasher must not freeze while it reads 200 jars, so each hash job gets its own thread.

A launcher that stays responsive while it works (hashing, downloading, syncing) runs the work on a **second thread**. `thread::spawn` starts one.

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hashing sodium.jar");
    21 * 2                       // the closure's value is the thread's result
});

let answer = handle.join().unwrap(); // wait for it: 42
```

- `spawn` takes a closure and returns a **`JoinHandle<T>`** right away. The thread runs on its own.
- `join()` blocks until the thread ends and gives `Result<T, _>`. `Err` means the thread **panicked**; the panic does not crash the main thread.
- Drop a handle without joining and nobody waits for the result.
- Start every thread **before** you join any, or you hash the jars one by one.

## Your turn

In `src/lib.rs`:

- `spawn_hash`: hash one `ModJar` on a new thread and return the handle.
- `hash_all`: one thread per jar, then join them all in folder order.
- `average_size`: divide inside a thread; if it panics (no jars), return `None`.
