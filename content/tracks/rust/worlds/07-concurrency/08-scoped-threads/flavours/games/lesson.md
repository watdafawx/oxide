**Why now:** the folder hasher has a list of jars and only wants to **read** it from several workers. Cloning it for each worker, or wrapping it in an `Arc`, is overkill. **`thread::scope`** guarantees every worker ends before the scope does, so workers may simply borrow.

```rust
use std::thread;

let sizes = vec![120u64, 340, 560, 80];
let (left, right) = sizes.split_at(2);

let total = thread::scope(|s| {
    let a = s.spawn(|| left.iter().sum::<u64>());   // borrows, no move
    let b = s.spawn(|| right.iter().sum::<u64>());
    a.join().unwrap() + b.join().unwrap()
});
println!("{total}");                                // 1100
```

- Scoped threads can borrow locals, because the scope joins them all before it returns.
- Threads that are not joined by hand are joined at the end of the scope.
- `&mut` works too, as long as the threads get **different** parts: `split_at_mut` hands out two non-overlapping halves.
- Add `move` when each thread needs its own loop value (like a `chunk`).

## Your turn

In `src/lib.rs`:

- `total_size`: two scoped threads, one per half of the slice.
- `biggest_jar`: one scoped thread per chunk, then the largest result.
- `hash_into`: two scoped threads, each writing `quick_hash` for its half of the jars into its half of `out`.
