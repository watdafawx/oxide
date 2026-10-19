`thread::spawn` demands `'static` data, so you had to `move` or `Arc` everything. **`thread::scope`** removes that: all its threads are **joined before the scope returns**, so they can safely borrow from the surrounding function.

```rust
use std::thread;

let words = vec!["tea", "coffee", "milk"];
let mut longest = 0;

thread::scope(|s| {
    s.spawn(|| {
        println!("{} words, borrowed, not moved", words.len());
    });
    s.spawn(|| {
        longest = words.iter().map(|w| w.len()).max().unwrap_or(0);
    });
});                              // both threads are joined here

println!("{longest}");           // 6
```

- `s.spawn(|| ...)` closures may borrow locals. Shared `&` borrows can overlap; a `&mut` must be the only one.
- No `Arc`, no `Mutex`, no `move` needed for read-only work.
- To give each thread its own **mutable** piece, split the data first (`split_at_mut`, `chunks_mut`). The borrow checker sees the pieces never overlap.
- `s.spawn` returns a handle: `join()` it inside the scope for a value. Un-joined threads are joined automatically at the end, and a panic in one is re-raised.

## Your turn

In `src/lib.rs`:

- `parallel_sum`: split the slice in two, sum each half on its own scoped thread.
- `max_of_chunks`: one scoped thread per chunk; return the biggest value overall.
- `fill_in_parallel`: two threads fill the two halves of a `&mut [u32]` with `index * 2`.
