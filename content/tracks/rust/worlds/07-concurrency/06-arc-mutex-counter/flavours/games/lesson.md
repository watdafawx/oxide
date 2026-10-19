**Why now:** a `Mutex` lives in one place, but the hash workers all need to reach it. `thread::spawn` wants each worker to **own** what it uses, so you hand every worker a clone of an **`Arc`**.

`Arc<T>` is `Rc<T>` with an atomic counter, so it may cross threads. Together: **`Arc<Mutex<T>>`**, shared state that many workers can change one at a time.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let done = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..4).map(|_| {
    let done = Arc::clone(&done);               // one handle per worker
    thread::spawn(move || {
        *done.lock().unwrap() += 1;             // one jar hashed
    })
}).collect();

for h in handles { h.join().unwrap(); }
println!("{}", *done.lock().unwrap());          // 4
```

- Clone the `Arc` **before** `spawn`, and move the clone in. Each worker owns one handle.
- Join all workers before reading the final value.
- To take the value back out: `Arc::try_unwrap(done).unwrap().into_inner().unwrap()`. It only works when no other clone is left.
- Tests run your code with a timeout. A guard held while you join is a deadlock.

## Your turn

In `src/lib.rs`:

- `hash_counter`: every worker adds to one shared counter.
- `collect_hashes`: worker `i` pushes a hash into one shared `Vec`.
- `loader_counts`: one worker per jar bumps that mod loader's count in a shared map.
