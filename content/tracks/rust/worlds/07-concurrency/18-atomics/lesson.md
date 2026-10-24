The "files done" counter now works with `Arc<Mutex<u32>>`, but locking, getting a guard and unlocking is a lot of machinery for adding 1 to a number. The processor has a cheaper way, the same one `Arc` uses for its own counter.

**One step that cannot be split.** An **atomic** type, such as `AtomicUsize` or `AtomicBool` from `std::sync::atomic`, is a number whose changes use special processor instructions. Read, add and write happen as one **indivisible** step: no other thread can get in between, so quest 08's lost update cannot happen. There is no lock to forget, no guard, and no deadlock.

**Change it through &.** `fetch_add(1, ..)` adds 1 and hands back the value from *before* the add; `load(..)` reads the current value. These methods take `&self`, not `&mut self`, so many threads may share one atomic by plain borrowing, with scoped threads, or own it through an `Arc`.

**Ordering, the second argument.** Every atomic call takes an `Ordering`. To go faster, processors and compilers may shuffle the order of memory reads and writes that do not depend on each other, and `Ordering` says how much of that is allowed around this one call. `Ordering::Relaxed` promises only that this one number is updated safely, which is all a counter needs.

**When it is not enough.** An atomic covers one number. Two values that must change together, like "files done" and "bytes done", still need a `Mutex` around both; two separate atomics could be read halfway between the two updates.

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let bytes_done = AtomicUsize::new(0);
let before = bytes_done.fetch_add(4_096, Ordering::Relaxed); // 0
let now = bytes_done.load(Ordering::Relaxed);                // 4096
```

**For the curious: Acquire, Release, SeqCst.** `Relaxed` says nothing about *other* memory. If thread A fills a buffer and then sets a "ready" flag, thread B may see the flag before the buffer, unless A writes with `Ordering::Release` and B reads with `Ordering::Acquire`: then everything A wrote before the flag is visible to B after it. `Ordering::SeqCst` goes further and gives every thread one single order of all `SeqCst` operations; it is the safe choice when unsure. Mara Bos' book *Rust Atomics and Locks* is the deep dive.

## Your turn

- In `file_done`, add 1 to the counter and return the value from before.
- In `count_in_threads`, let `threads` scoped threads each call `file_done` `per_thread` times on one shared `AtomicUsize`, then return its value.
