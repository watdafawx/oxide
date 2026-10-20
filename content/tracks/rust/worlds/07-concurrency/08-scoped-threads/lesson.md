**Why now:** the hasher can hash slices of one big file list without cloning it into an `Arc`, because scoped threads may borrow it.

`thread::spawn` demands `'static` data, so you had to `move` or `Arc` everything. **`thread::scope`** removes that: all its threads are **joined before the scope returns**, so they can safely borrow from the surrounding function.

```rust
use std::thread;

let jars = vec!["sodium.jar", "lithium.jar", "iris.jar"];
let mut longest = 0;

thread::scope(|s| {
    s.spawn(|| {
        println!("{} jars, borrowed, not moved", jars.len());
    });
    s.spawn(|| {
        longest = jars.iter().map(|j| j.len()).max().unwrap_or(0);
    });
});                              // both threads are joined here

println!("{longest}");           // 11
```

- `s.spawn(|| ...)` closures may borrow locals. Shared `&` borrows can overlap; a `&mut` must be the only one.
- No `Arc`, no `Mutex`, no `move` needed for read-only work.
- Cutting a slice: **`data.split_at(2)`** gives two slices (first 2 items, the rest); **`data.chunks(3)`** yields slices of up to 3 items each, so `.map(|chunk| s.spawn(move || ...))` starts one thread per chunk (`move` copies the loop's `chunk` into its thread; the data itself is still only borrowed). `.iter().copied().max()` turns `&i32` items into `i32` and gives an `Option`.
- To give each thread its own **mutable** piece, split the data first (`split_at_mut`, `chunks_mut`). The borrow checker sees the pieces never overlap. Change items in place with **`for x in piece.iter_mut() { *x = ... }`** (`x` is a `&mut`, so write through `*x`).
- `s.spawn` returns a handle: `join()` it inside the scope for a value. Un-joined threads are joined automatically at the end, and a panic in one is re-raised.

## Your turn

In `src/lib.rs`:

- `parallel_sum`: split the slice in two, sum each half on its own scoped thread.
- `max_of_chunks`: one scoped thread per chunk; return the biggest value overall.
- `fill_in_parallel`: two threads fill the two halves of a `&mut [u32]` with `index * 2`.
