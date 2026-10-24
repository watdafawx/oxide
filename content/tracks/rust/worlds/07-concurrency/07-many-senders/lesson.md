The hasher runs several workers, and each one reports its results into the same channel so one place can collect them all. A channel allows that by design, but there is one trap that makes the collecting loop wait forever.

**Many in, one out.** `mpsc` stands for **multiple producer, single consumer**: many `Sender`s may feed one `Receiver`. `tx.clone()` makes another `Sender` for the same channel. Clone once per worker, inside the loop, and move the clone into that worker's closure.

**Try the obvious thing.** Clone for every worker, start them, then loop over `rx`. The values all arrive, and then the loop never ends. The channel closes only when *every* `Sender` is gone, and the original `tx` is still alive in your own function, which will never send anything.

**Drop the original.** After the loop that starts the workers, call `drop(tx)`, which you met in the smart-pointers world. Now the only `Sender`s left are the clones inside the workers, and each one is dropped when its worker finishes. When the last worker ends, the channel closes and the `for` over `rx` stops.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
for part in ["header", "body"] {
    let tx = tx.clone();                 // this thread's own Sender
    thread::spawn(move || tx.send(part.len()).unwrap());
}
drop(tx);                                // without this, the loop below never ends
let lengths: Vec<usize> = rx.iter().collect();
```

**For the curious: a bounded channel.** `mpsc::channel()` has no size limit, so a fast producer can fill memory while a slow consumer falls behind. `mpsc::sync_channel(8)` holds at most 8 values: when it is full, `send` waits until the receiver takes one. That waiting, called **backpressure**, keeps a file lister from running far ahead of the hashers.

## Your turn

- In `fan_out`, give each of the `workers` threads its own clone of the `Sender`; worker `id` sends `id * 10` once.
- Drop the original `Sender` before you return the `Receiver`.
