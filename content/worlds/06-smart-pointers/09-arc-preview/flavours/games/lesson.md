**Why now:** a preview of a big factory costs out every recipe, and you want two threads to split the list. `Rc` is not allowed across threads (its counter is not atomic). **`Arc<T>`** is the thread-safe twin: same API, `Arc::clone`, `Arc::strong_count`, slightly slower counter.

```rust
use std::sync::Arc;
use std::thread;

let costs = Arc::new(vec![3, 5, 8]);
let c = Arc::clone(&costs);

let handle = thread::spawn(move || c.iter().sum::<u32>());
let total = handle.join().unwrap();
println!("{total}"); // 16
```

- `move` hands the closure its own `Arc` handle.
- `Arc<T>` is read-only. Shared *mutation* needs `Arc<Mutex<T>>`: that is world 07.
- `join()` waits for the thread and returns its result.

Keep it small: this is a preview.

## Your turn

In `src/lib.rs`, `total_cost`: wrap the recipe list in an `Arc`, spawn two threads (first half, second half), join both and add the results.
