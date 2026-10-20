**Why now:** once the hasher works with hand-built threads, rayon turns the same "hash every jar" loop into one `par_iter()` call.

Doing the splitting, spawning, joining and merging by hand gets old. **rayon** is a crate that does it for you: keep your iterator chain, swap one word, and the work runs on a pool of threads (one per CPU core).

```rust
use rayon::prelude::*;

let jars = vec!["sodium.jar", "lithium.jar", "iris.jar"];

let lengths: Vec<usize> = jars
    .par_iter()                      // was: .iter()
    .map(|j| j.len())
    .collect();                      // order is preserved: [10, 11, 8]

let total: usize = (1..=100_u32)
    .into_par_iter()                 // was: .into_iter()
    .map(|n| n as usize)
    .sum();                          // 5050
```

- Filtering works the same: `(0..10_u32).into_par_iter().filter(|&n| n % 2 == 0).count()` is `5` (the closure gets a reference, so `|&n|` unpacks it).
- `par_iter()` borrows, `into_par_iter()` consumes, `par_iter_mut()` borrows mutably.
- Most adapters you know work: `map`, `filter`, `sum`, `count`, `max`, `collect`, `for_each`, `reduce`.
- The closure runs on several threads, so it must be `Send + Sync`: no `Rc`, no `RefCell`, no mutating a captured variable. Return values instead.
- It only pays off when each item is real work. For tiny closures a plain `iter()` is faster.
- This quest has an external crate: `Cargo.toml` already lists `rayon`, the first build downloads it.

## Your turn

In `src/lib.rs`:

- `sum_of_squares`: sum of `n * n` over a slice, in parallel.
- `parallel_lengths`: the `len()` of each string, in parallel, in order.
- `count_primes`: how many numbers in `0..limit` are prime (`is_prime` is given).
