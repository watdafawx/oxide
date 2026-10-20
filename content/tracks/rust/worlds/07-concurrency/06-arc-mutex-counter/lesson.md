**Why now:** threads that each own a clone of one shared total are how the hasher adds up "bytes hashed so far" from every worker.

A `Mutex` alone has one owner, but each spawned thread needs to own something. **`Arc<Mutex<T>>`** fixes both: `Arc` gives every thread an owner, `Mutex` makes the changes safe. It is `Rc<RefCell<T>>` for threads.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let total = Arc::new(Mutex::new(0_u64));
let mut handles = Vec::new();

for jar_bytes in [9_500, 20_000, 4_500] {
    let total = Arc::clone(&total);          // one clone per thread
    handles.push(thread::spawn(move || {
        *total.lock().unwrap() += jar_bytes;
    }));
}
for h in handles {
    h.join().unwrap();                       // wait for all of them
}
println!("{}", total.lock().unwrap());       // 34000
```

- Clone the `Arc` **before** `spawn` and move the clone in. Shadowing the name keeps it tidy.
- Join first, read after. Locking in the parent while threads still run is legal but gives a partial answer.
- Once every other clone is dropped, `Arc::try_unwrap(arc).unwrap()` gives the `Mutex` back and `.into_inner().unwrap()` the value inside.
- Shared map: lock, then use the map as usual, e.g. `*map.lock().unwrap().entry(key).or_insert(0) += 1;`.
- Do the slow work **outside** the lock and hold the guard only for the update.

## Your turn

In `src/lib.rs`:

- `count_with_threads`: `threads` threads each add 1 to a shared counter `per_thread` times.
- `collect_squares`: thread `i` pushes `i * i` into a shared Vec; return it sorted.
- `word_counts`: one thread per word adds to a shared `BTreeMap<String, usize>`.
