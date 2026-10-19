A `Mutex` alone has one owner, but each spawned thread needs to own something. **`Arc<Mutex<T>>`** fixes both: `Arc` gives every thread an owner, `Mutex` makes the changes safe. It is `Rc<RefCell<T>>` for threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let total = Arc::new(Mutex::new(0.0_f64));
let mut handles = Vec::new();

for price in [9.5, 20.0, 4.5] {
    let total = Arc::clone(&total);          // one clone per thread
    handles.push(thread::spawn(move || {
        *total.lock().unwrap() += price;
    }));
}
for h in handles {
    h.join().unwrap();                       // wait for all of them
}
println!("{}", total.lock().unwrap());       // 34
```

- Clone the `Arc` **before** `spawn` and move the clone in. Shadowing the name keeps it tidy.
- Join first, read after. Locking in the parent while threads still run is legal but gives a partial answer.
- Once every other clone is dropped, `Arc::try_unwrap(arc)` gives the `Mutex` back and `into_inner()` the value.
- Do the slow work **outside** the lock and hold the guard only for the update.

## Your turn

In `src/lib.rs`:

- `count_with_threads`: `threads` threads each add 1 to a shared counter `per_thread` times.
- `collect_squares`: thread `i` pushes `i * i` into a shared Vec; return it sorted.
- `word_counts`: one thread per word adds to a shared `BTreeMap<String, usize>`.
