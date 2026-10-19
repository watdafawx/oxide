**Why now:** the user closes the launcher with five downloads running. You must tell every worker to stop, wait until they really did, and only then exit. A **`CancellationToken`** is the shared "stop" signal for that.

```rust
use tokio_util::sync::CancellationToken;

let token = CancellationToken::new();

let worker_token = token.clone();               // clones share one signal
let handle = tokio::spawn(async move {
    loop {
        tokio::select! {
            _ = worker_token.cancelled() => return "stopped",
            _ = download_next_chunk() => {}
        }
    }
});

token.cancel();                                  // every clone sees it at once
let result = handle.await.unwrap();              // "stopped"
```

- `token.cancelled()` is a future that completes when someone calls `cancel()`. Put it in a `select!` next to the real work.
- The worker checks the token at each `.await`, so it stops within one wait, not after the whole job.
- Graceful shutdown is two steps: **cancel**, then **await every handle**. Dropping a `JoinHandle` does not stop a task.
- This is the pattern the boss uses for its Cancel button.

## Your turn

In `src/lib.rs`:

- `spawn_worker`: a task that adds 1 to `chunks` every 5 ms and returns `"stopped"` when the token is cancelled.
- `shutdown`: cancel the token, await every worker and return how many said `"stopped"`.
