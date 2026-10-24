Now put the two together. A `Mutex` makes changes take turns, but each spawned thread needs to *own* what it uses, and a `Mutex` has a single owner. An `Arc` gives many owners but only read access. Stacked, they give the hasher one "jars hashed" counter that every worker owns and may change.

**Stack them.** `Arc<Mutex<T>>`: the `Arc` gives every thread its own owner of the same mutex, and the `Mutex` inside makes the changes take turns. Read the type from the outside in: "shared, lockable T". It is the thread version of the `Rc<RefCell<T>>` from the smart-pointers world.

**One clone per thread.** Make the `Arc` once, then inside the loop clone it and move the clone into that thread's closure. Each thread locks its clone, changes the value, and drops the guard. All the clones point at the same mutex, so all the changes land in one place.

**Join, then read.** Reading while threads still run is legal, but you get a number somewhere in the middle. Join every handle first; then the main thread locks once more and reads the final value. Copy it out (`*` for a number, `.clone()` for a `Vec`) so the guard can go.

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let total = Arc::new(Mutex::new(0_u64));
let mut handles = Vec::new();
for size in [9_500, 20_000, 4_500] {     // jar sizes in KB
    let mine = Arc::clone(&total);
    handles.push(thread::spawn(move || *mine.lock().unwrap() += size));
}
for handle in handles {
    handle.join().unwrap();
}
println!("{}", *total.lock().unwrap());     // 34000
```

## Your turn

- In `count_with_workers`, start `threads` workers that each add 1 to a shared counter `per_thread` times; join them and return the count.
- In `hashed_jars`, each worker pushes `"hashed <jar>"` into one shared `Vec`; join, then return a sorted copy.
