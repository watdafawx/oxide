**Why now:** each worker thread has to send its `(file, hash)` result back to the main thread, and a channel is the pipe the hasher uses for that.

A **channel** moves values from one thread to another without sharing memory: threads talk, they don't touch each other's data. It is how a background job reports progress to the UI.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();          // Sender, Receiver

for file_id in 0..3 {
    let tx = tx.clone();                 // mpsc = multi producer, single consumer
    thread::spawn(move || tx.send(file_id).unwrap());
}
drop(tx);                                // drop the original sender

for got in rx {                          // loops until ALL senders are dropped
    println!("got {got}");
}
```

- A `Receiver` is an iterator: `rx.iter().collect()` gathers everything into a `Vec`, ending when all senders are gone.
- `send` moves the value into the channel. `recv` blocks until one arrives.
- The receiver stops (`Err` / end of the `for`) only when **every** `Sender` is dropped. A forgotten `tx` means waiting forever.
- Tests here use `recv_timeout`, so a mistake shows as a failure, not a freeze.

## Your turn

In `src/lib.rs`:

- `send_all`: spawn a thread that sends each item, return the `Receiver`.
- `fan_out`: spawn `workers` producers, each sending `id * 10`. Close the channel correctly.
- `drain`: collect everything a `Receiver` yields.
