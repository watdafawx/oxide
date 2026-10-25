The "jars hashed" counter now works with `Arc<Mutex<u32>>`, but locking, getting a guard and unlocking is a lot of machinery for adding 1 to a number. The processor has a cheaper way, the same one `Arc` uses for its own counter.

**One step that cannot be split.** An **atomic** type, such as `AtomicUsize` or `AtomicBool` from `std::sync::atomic`, is a number whose changes use special processor instructions. Read, add and write happen as one **indivisible** step: no other thread can get in between, so quest 08's lost update cannot happen. There is no lock to forget, no guard, and no deadlock.

**Change it through &.** `fetch_add(1, ..)` adds 1 and hands back the value from *before* the add; `load(..)` reads the current value. These methods take `&self`, not `&mut self`, so many threads may share one atomic by plain borrowing, with scoped threads, or own it through an `Arc`.

**Ordering, the second argument.** Every atomic call takes an `Ordering`. To go faster, processors and compilers may shuffle the order of memory reads and writes that do not depend on each other, and `Ordering` says how much of that is allowed around this one call. `Ordering::Relaxed` promises only that this one number is updated safely, which is all a counter needs.

**When it is not enough.** An atomic covers one number. Two values that must change together, like "jars hashed" and "bytes hashed", still need a `Mutex` around both; two separate atomics could be read halfway between the two updates.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let bytes_done = AtomicUsize::new(0);
let before = bytes_done.fetch_add(4_096, Ordering::Relaxed); // 0
let now = bytes_done.load(Ordering::Relaxed);                // 4096
```

**For the curious: Acquire, Release, SeqCst.** `Relaxed` says nothing about *other* memory. If a worker fills a buffer with a jar's bytes and then sets a "ready" flag, another thread may see the flag before the buffer, unless the worker writes with `Ordering::Release` and the reader reads with `Ordering::Acquire`: then everything the worker wrote before the flag is visible to the reader after it. `Ordering::SeqCst` goes further and gives every thread one single order of all `SeqCst` operations; it is the safe choice when unsure. Mara Bos' book *Rust Atomics and Locks* is the deep dive.

## Your turn

- In `jar_done`, add 1 to the counter and return the value from before.
- In `count_in_workers`, let `threads` scoped workers each call `jar_done` `per_thread` times on one shared `AtomicUsize`, then return its value.
