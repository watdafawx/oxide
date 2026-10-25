A user reports that while one episode is being checked, every other download freezes. The checking code is correct, it just doesn't wait the async way. Tokio's few threads are shared by every task, and one task held its thread.

**Giving the thread back.** A worker thread runs one task at a time. The task gives the thread back only at an `.await` that isn't ready yet; then the runtime runs a different task there. Between two awaits a task has the thread to itself.

**What blocking is.** A call **blocks** when it keeps the thread until it is done, without an `.await`. `std::thread::sleep(5 s)`, a big file read with `std::fs`, or a loop that hashes for two seconds all block. The thread can't run anything else meanwhile, so every task waiting for that thread stalls, and in a test it can look like a hang.

```rust
async fn retry_later() {
    tokio::time::sleep(Duration::from_secs(5)).await; // gives the thread back
}

async fn retry_frozen() {
    std::thread::sleep(Duration::from_secs(5)); // holds the thread for 5 s
}
```

**Use the async twin.** Most waiting calls have an async version that gives the thread back: `tokio::time::sleep` for `std::thread::sleep`, `tokio::fs` for `std::fs`, tokio's channels and locks for the ones in `std`. Inside async code, reach for those.

**CPU work goes elsewhere.** A long calculation has nothing to await, so it can't give the thread back. Run it on a thread meant for blocking work: `tokio::task::spawn_blocking(move || checksum(&bytes)).await` hands a closure to a separate pool of threads and lets you await its result. World 12 uses it; rayon from world 07 is the other choice.

**A rule of thumb.** If a call can take more than a few milliseconds and has no `.await`, it does not belong directly in async code.

## Your turn

- Answer the two questions. No code this time.
