Five mod downloads run as tasks, and the launcher shows "sodium.jar 40%". Every task could lock one shared list, but there is a simpler shape: each task sends a short message, and the UI task collects them. That is a channel, the async cousin of world 07's.

**A queue between tasks.** `let (tx, mut rx) = mpsc::channel(8);` makes a channel: `tx` is the sending end, `rx` the receiving end, and up to 8 messages can wait in between. **mpsc** means multiple producers, single consumer: many senders, one receiver.

**Full means wait.** If 8 messages are already waiting, `tx.send(x).await` pauses that task until the reader takes one. That is **backpressure**: a slow UI slows the fast downloads down instead of letting the queue eat all the memory. A **bounded** channel, one with a size, gives you that for free.

**One sender each.** `tx.clone()` makes another sender for the same channel. Give every task its own clone, moved into its `async move` block, and the receiver hears them all.

```rust
use tokio::sync::mpsc;

let (tx, mut rx) = mpsc::channel::<String>(8);
let pack_tx = tx.clone();
tokio::spawn(async move { pack_tx.send("modpack checked".to_string()).await.unwrap(); });
drop(tx); // the last sender we hold

while let Some(line) = rx.recv().await { // None once every sender is gone
    println!("{line}");
}
```

**When does it end?** `rx.recv().await` gives `Some(msg)`, or `None` once every sender has been dropped and the queue is empty. The `while let` loop from world 06 stops on that `None`. Forget to drop your own original `tx` and one sender stays alive forever, so the loop never ends.

**Async on both ends.** `std::sync::mpsc` from world 07 blocks a thread while it waits; tokio's `send` and `recv` give the thread back at the `.await`, which is what async code needs.

## Your turn

- In `drain_log`, collect every progress line until the channel closes.
- In `gather_progress`, spawn one task per id that sends `id * 10` on its own sender, then collect everything and sort it. It must also finish when there are no downloads at all.
