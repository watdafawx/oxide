Two facts explain most async bugs. Read once.

**1. Futures are lazy.**
Calling `fetch()` does not fetch anything. It builds a future: a paused state machine. The runtime **polls** it when you `.await` it (or `spawn` it, or put it in `join!`). Never polled, never run.

**2. Waiting must not block.**
An async runtime has a few worker threads, each juggling many tasks. A task gives its thread back at every `.await`. If you call something that **blocks** (like `std::thread::sleep`, a big loop, a sync file read), the thread is stuck and every other task on it stalls.

```rust
async fn good() {
    tokio::time::sleep(Duration::from_secs(1)).await; // thread is free meanwhile
}

async fn bad() {
    std::thread::sleep(Duration::from_secs(1)); // thread frozen, nothing else runs there
}

async fn forgot() {
    good(); // warning: unused future. Nothing happens!
}
```

Rule of thumb: inside `async`, use the async version (`tokio::time::sleep`, `tokio::fs`). For heavy CPU work, use `tokio::task::spawn_blocking`.

## Your turn

- Answer the two questions. No code this time.
