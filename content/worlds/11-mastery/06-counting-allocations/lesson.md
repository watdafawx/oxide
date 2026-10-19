Allocation is usually the first cost to check in slow Rust code. You can count it exactly with a **global allocator** wrapper.

`#[global_allocator]` replaces the program's allocator. Yours forwards to `System` and bumps a counter first. Test code can then ask "how many times did this closure allocate?".

```rust
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::Cell;

thread_local! { static COUNT: Cell<usize> = const { Cell::new(0) }; }

struct Counting;
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        COUNT.with(|c| c.set(c.get() + 1));
        unsafe { System.alloc(l) }
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        unsafe { System.dealloc(p, l) }
    }
}
```

The counter is per thread, so parallel tests do not disturb each other. It is a plain count, never a stopwatch, so the result is the same on every machine.

A growing `String` or `Vec` reallocates every time it runs out of room. If you know the size, reserve it once with **`with_capacity`**.

## Your turn

The wrapper is already in `tests/allocations.rs`. In `src/lib.rs`:

- `join_words`: join words with single spaces using one allocation.
- `dedup_sorted`: remove consecutive duplicates from a sorted slice using one allocation.
