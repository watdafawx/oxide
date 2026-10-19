For a plain counter or flag, a `Mutex` is overkill. **Atomics** (`AtomicUsize`, `AtomicBool`, ...) let many threads change one number safely using a single CPU instruction. No lock, no guard, no deadlock.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

static HITS: AtomicUsize = AtomicUsize::new(0);   // a global needs no Mutex

let before = HITS.fetch_add(1, Ordering::Relaxed); // returns the OLD value
let now = HITS.load(Ordering::Relaxed);

// "set to 10 only if it is still 0":
let _ = HITS.compare_exchange(0, 10, Ordering::SeqCst, Ordering::SeqCst);
```

- Methods take `&self`, so you can share an atomic by `&` (scoped threads, `static`) or `Arc`.
- `fetch_add`, `swap`, `load`, `store`, `compare_exchange`: each is one indivisible step.
- **Ordering** says how much the CPU may reorder *other* memory accesses around the operation. `Relaxed`: only this one variable is atomic, fine for counters nobody uses to coordinate. `SeqCst`: all threads see one single global order, the safe default when you are unsure. (`Acquire`/`Release` sit between; skip them for now.)
- Atomics fit one number. Updating two related fields together still needs a `Mutex`.

## Your turn

In `src/lib.rs`:

- `increment`: `fetch_add` and return the previous value.
- `count_in_threads`: lock-free version of the Arc<Mutex> counter.
- `claim_once`: a flag that only the first caller can flip (`compare_exchange`).
