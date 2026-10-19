**Why now:** many download tasks run at once, but the launcher UI wants one stream of "sodium.jar 40%" lines. Tasks should not share a list behind a lock; they **send messages** to one receiver through a channel.

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<u32>(4);   // buffer for 4 pending messages

for id in 0..3 {
    let tx = tx.clone();                       // one sender per download task
    tokio::spawn(async move { tx.send(id * 10).await.unwrap(); });
}
drop(tx);                                      // the original sender must go too

while let Some(percent) = rx.recv().await {    // None once ALL senders are dropped
    println!("{percent}%");
}
```

- `send(..).await` waits when the buffer is full. That is **backpressure**: a slow UI slows fast downloads instead of eating memory.
- `recv().await` returns `None` only when every `Sender` is dropped. A forgotten `tx` means waiting forever.
- Unlike `std::sync::mpsc`, both ends are async: they never block the thread.
- Tests use `timeout`, so a mistake shows as a failure, not a freeze.

## Your turn

In `src/lib.rs`:

- `drain_log`: collect every message from the receiver until it closes.
- `gather_progress`: spawn one task per download id; task `id` sends `id * 10`. Return everything received, sorted ascending.
