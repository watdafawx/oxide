`join` gives you one answer when a thread ends. The hasher wants more: a thread that lists `mods/` should hand over each jar name as soon as it finds it, so other threads can start hashing while the listing goes on. That needs a pipe between threads.

**A pipe with two ends.** A **channel** is that pipe, like a conveyor belt between two machines. `mpsc::channel()` from `std::sync` makes one and hands back both ends as a pair: a `Sender`, usually called `tx`, and a `Receiver`, called `rx`. Values go in at `tx` and come out at `rx`, in the order they were sent. The threads never touch each other's variables; they only pass values along.

**Send moves.** `tx.send(value)` moves the value into the pipe, so ownership goes with it to the other thread. You move `tx` into the producing thread with a `move` closure and keep `rx`. `send` returns a `Result`, an `Err` only if the receiving end is gone, so `unwrap()` it.

**Receive waits.** `rx.recv()` waits until a value arrives. Even simpler, a `Receiver` works in a `for` loop: `for jar in rx` takes each value as it comes in and waits between them.

**When does it end?** The loop stops when the channel is **closed**, which happens once every `Sender` is dropped. Here that is automatic: when the thread finishes, its closure and the `tx` inside it are dropped. The tests use a time limit, so a channel that never closes fails the test instead of freezing it.

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    for size in [120, 4_000, 75] {
        tx.send(size).unwrap();
    }
});                                // thread ends: tx dropped, channel closes

for size in rx {
    println!("got {size}");        // 120, 4000, 75
}
```

## Your turn

- In `stream_jars`, make a channel, spawn a thread that sends every jar name, and return the `Receiver` right away.
- In `drain_sizes`, collect every value the `Receiver` yields into a `Vec`.
