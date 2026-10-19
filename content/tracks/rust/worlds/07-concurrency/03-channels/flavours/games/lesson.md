**Why now:** each hash worker has to send its `(jar, hash)` result back to the main thread, and a channel is the pipe the hasher uses for that.

A **channel** moves values from one thread to another without sharing memory: threads talk, they don't touch each other's data. It is how a background job reports progress to the UI.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();          // Sender, Receiver

for jar_id in 0..3 {
    let tx = tx.clone();                 // mpsc = multi producer, single consumer
    thread::spawn(move || tx.send(jar_id).unwrap());
}
drop(tx);                                // drop the original sender

for hashed in rx {                       // loops until ALL senders are dropped
    println!("hashed jar {hashed}");
}
```

- `send` moves the value into the channel. `recv` blocks until one arrives.
- The receiver stops (`Err` / end of the `for`) only when **every** `Sender` is dropped. A forgotten `tx` means waiting forever.
- Tests here use `recv_timeout`, so a mistake shows as a failure, not a freeze.

## Your turn

In `src/lib.rs`:

- `stream_names`: spawn a thread that sends each jar name, return the `Receiver`.
- `hash_workers`: spawn `workers` producers; worker `id` sends a `HashResult` with `hash = id * 10`. Close the channel correctly.
- `drain_results`: collect everything a `Receiver` yields.
