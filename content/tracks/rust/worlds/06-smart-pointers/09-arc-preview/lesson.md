`Rc` is not allowed across threads (its counter is not atomic). **`Arc<T>`** is the thread-safe twin: same API, `Arc::clone`, `Arc::strong_count`, slightly slower counter.

```rust
use std::sync::Arc;
use std::thread;

let data = Arc::new(vec![1, 2, 3]);
let d = Arc::clone(&data);

let handle = thread::spawn(move || d.iter().sum::<i32>());
let total = handle.join().unwrap();
println!("{total}"); // 6
```

- `move` hands the closure its own `Arc` handle.
- `Arc<T>` is read-only. Shared *mutation* needs `Arc<Mutex<T>>`: that is world 07.
- `join()` waits for the thread and returns its result.

Keep it small: this is a preview.

- A slice of an `Arc<Vec<T>>` works like a slice of a `Vec`: `data[..mid]` is the first half, `data[mid..]` the rest. Each thread gets its own `Arc::clone` and uses it with `move`.

## Your turn

In `src/lib.rs`, `parallel_sum`: wrap the Vec in an `Arc`, spawn two threads (first half, second half), join both and add the results.
