**Why now:** when the user hits Cancel on the downloader, every spawned download task must stop and be awaited, not just dropped.

Dropping a future cancels it, but background tasks you **spawned** keep running unless you tell them.

- **`CancellationToken`** (crate `tokio-util`) is a shared stop button. Clone it into each task.
- `token.cancel()` presses it. `token.cancelled().await` completes once it is pressed.
- Inside a task: `select!` between your work and `token.cancelled()`.
- Graceful shutdown = cancel, then **await the handles** so you know everything really stopped.

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();
let t = token.clone();
let handle = tokio::spawn(async move {
    tokio::select! {
        _ = t.cancelled() => "stopped",
        _ = tokio::time::sleep(std::time::Duration::from_secs(60)) => "finished",
    }
});
token.cancel();
assert_eq!(handle.await.unwrap(), "stopped");
```

`select!` is what makes a long `sleep` or download interruptible. A loop that only checks a flag between slow steps reacts late.

## Your turn

In `src/lib.rs`:

- `spawn_worker`: a task that adds 1 to `ticks` every 5 ms until the token is cancelled, then returns `"stopped"`.
- `shutdown`: cancel the token, await all handles, count the `"stopped"` results.
