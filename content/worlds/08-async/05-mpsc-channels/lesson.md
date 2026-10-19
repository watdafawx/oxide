A download task needs to tell the UI "file 3 done". Tasks should not share variables for that; they send **messages** through a channel.

- `let (tx, mut rx) = mpsc::channel(8);` makes a **bounded** queue of 8. If it is full, `tx.send(x).await` waits: free backpressure.
- **m**ultiple **p**roducers: `tx.clone()` gives every task its own sender. One receiver.
- `rx.recv().await` returns `Some(msg)` or `None` once **every** sender is dropped.

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<u32>(8);
for i in 0..3 {
    let tx = tx.clone();
    tokio::spawn(async move { tx.send(i).await.unwrap(); });
}
drop(tx); // our own sender must go too, or the loop never ends

while let Some(n) = rx.recv().await {
    println!("got {n}");
}
```

The classic bug: forgetting `drop(tx)`. The receiver waits forever for a sender that is still alive.

## Your turn

In `src/lib.rs`:

- `drain`: collect every message from a receiver until it closes.
- `gather`: spawn one worker per id (`0..workers`) that sends `id * 10`, then collect and return the values sorted.
