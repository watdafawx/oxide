**Why now:** with `join!` you know the downloads up front. A real queue grows while it runs, so each download becomes its own **task**: tokio runs them side by side and gives you a handle for each.

```rust
use tokio::task::JoinHandle;

fn spawn_fetch(kb: u64) -> JoinHandle<u64> {
    tokio::spawn(async move { kb * 1024 })   // runs on its own, right away
}

let handle = spawn_fetch(2);
let bytes = handle.await.unwrap();            // 2048
```

- `tokio::spawn` takes a future and returns a **`JoinHandle<T>`** immediately. The task is already running.
- Awaiting the handle gives `Result<T, JoinError>`. `Err` means the task **panicked**: your launcher survives it.
- The future must own what it uses (`async move`) because the task may outlive the function.
- Spawn all tasks **before** you await any, or you download one at a time again.

## Your turn

In `src/lib.rs`:

- `spawn_fetch`: spawn a task that returns `kb * 1024`.
- `fetch_all`: one task per file size, results in the same order.
- `average_kb`: divide inside a spawned task; if it panics (zero files), return `None`.
