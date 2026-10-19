**Why now:** your mod folder hasher must not freeze while it reads 200 jars, so each hash job needs its own thread.

Every app that stays responsive while it works (downloads, hashing, syncing) runs work on a **second thread**. `thread::spawn` starts one.

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hashing sodium.jar");
    21 * 2                       // the closure's value is the thread's result
});

let answer = handle.join().unwrap(); // wait for it: 42
```

- `spawn` takes a closure and returns a **`JoinHandle<T>`** right away. The thread runs on its own.
- `join()` blocks until the thread ends and gives `Result<T, _>`. `Err` means the thread **panicked**; the panic does not crash your main thread.
- Drop a handle without joining and the thread is detached: nobody waits for its result.
- Start every thread **before** you join any, or you just run them one by one.

## Your turn

In `src/lib.rs`:

- `spawn_square`: spawn a thread that returns `n * n` and return its handle.
- `square_all`: one thread per number, then join them all in order.
- `safe_div`: divide inside a thread; if it panics, return `None`.
