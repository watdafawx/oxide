When a worker finds an unreadable file, it writes an error report. If three workers hit errors at once, the report must still be written exactly once. A shared `AtomicBool` that says "someone is on it" sounds right, but how the workers use it matters.

**Try the obvious thing.** Each worker does `if !flag.load(..) { flag.store(true, ..); write_report(); }`. Both calls are atomic, but there are two of them. Two workers can both load `false` before either stores `true`, and both write the report. There is no data race here; it is the **race condition** from quest 08, and the compiler cannot see it.

**Check and change in one step.** `flag.compare_exchange(false, true, ..)` means: "if the value is still `false`, make it `true`", as one indivisible step. Exactly one thread can win that. The winner gets `Ok(false)`, the old value; every other thread gets `Err(true)`, the value it found instead. `.is_ok()` turns that into "did I win?".

**Two orderings.** `compare_exchange` takes two `Ordering`s: one for when the change happens and one for when it does not. `Ordering::SeqCst` for both is the safe choice from the last quest.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let next_id = AtomicUsize::new(7);
let moved = next_id.compare_exchange(7, 8, Ordering::SeqCst, Ordering::SeqCst); // Ok(7)
let again = next_id.compare_exchange(7, 8, Ordering::SeqCst, Ordering::SeqCst); // Err(8)
```

## Your turn

- In `claim_report`, flip the flag from `false` to `true` with `compare_exchange`, and return whether this caller was the one that did it.
